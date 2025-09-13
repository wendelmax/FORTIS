// FORTIS Frontend - Página de Eleições

import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import { Plus, Vote, Calendar, Users, Eye } from 'lucide-react';
import { useElections } from '../hooks/useElections';
import { Election } from '../types';

const ElectionsPage: React.FC = () => {
  const { elections, loading, error } = useElections();
  const [filter, setFilter] = useState('all');

  const getStatusBadge = (status: string) => {
    const statusConfig = {
      draft: { label: 'Rascunho', className: 'badge-gray' },
      active: { label: 'Ativa', className: 'badge-success' },
      paused: { label: 'Pausada', className: 'badge-warning' },
      completed: { label: 'Concluída', className: 'badge-primary' },
      cancelled: { label: 'Cancelada', className: 'badge-error' },
    };
    
    const config = statusConfig[status as keyof typeof statusConfig] || statusConfig.draft;
    return <span className={`badge ${config.className}`}>{config.label}</span>;
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString('pt-BR', {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  const filteredElections = elections.filter(election => {
    if (filter === 'all') return true;
    return election.status === filter;
  });

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary-600"></div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="bg-error-50 border border-error-200 rounded-md p-4">
        <p className="text-sm text-error-800">{error}</p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Eleições</h1>
          <p className="mt-1 text-sm text-gray-500">
            Gerencie todas as eleições do sistema
          </p>
        </div>
        <button className="btn-primary">
          <Plus className="h-5 w-5 mr-2" />
          Nova Eleição
        </button>
      </div>

      {/* Filters */}
      <div className="flex space-x-4">
        <button
          onClick={() => setFilter('all')}
          className={`px-4 py-2 text-sm font-medium rounded-md ${
            filter === 'all'
              ? 'bg-primary-100 text-primary-900'
              : 'text-gray-500 hover:text-gray-700'
          }`}
        >
          Todas ({elections.length})
        </button>
        <button
          onClick={() => setFilter('active')}
          className={`px-4 py-2 text-sm font-medium rounded-md ${
            filter === 'active'
              ? 'bg-primary-100 text-primary-900'
              : 'text-gray-500 hover:text-gray-700'
          }`}
        >
          Ativas ({elections.filter(e => e.status === 'active').length})
        </button>
        <button
          onClick={() => setFilter('completed')}
          className={`px-4 py-2 text-sm font-medium rounded-md ${
            filter === 'completed'
              ? 'bg-primary-100 text-primary-900'
              : 'text-gray-500 hover:text-gray-700'
          }`}
        >
          Concluídas ({elections.filter(e => e.status === 'completed').length})
        </button>
      </div>

      {/* Elections Grid */}
      {filteredElections.length === 0 ? (
        <div className="text-center py-12">
          <Vote className="mx-auto h-12 w-12 text-gray-400" />
          <h3 className="mt-2 text-sm font-medium text-gray-900">
            Nenhuma eleição encontrada
          </h3>
          <p className="mt-1 text-sm text-gray-500">
            {filter === 'all' 
              ? 'Comece criando uma nova eleição.'
              : 'Nenhuma eleição com este status.'
            }
          </p>
          <div className="mt-6">
            <button className="btn-primary">
              <Plus className="h-5 w-5 mr-2" />
              Nova Eleição
            </button>
          </div>
        </div>
      ) : (
        <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
          {filteredElections.map((election) => (
            <div key={election.id} className="card hover:shadow-lg transition-shadow">
              <div className="card-body">
                <div className="flex items-start justify-between">
                  <div className="flex-1">
                    <h3 className="text-lg font-medium text-gray-900">
                      {election.title}
                    </h3>
                    {election.description && (
                      <p className="mt-1 text-sm text-gray-500 line-clamp-2">
                        {election.description}
                      </p>
                    )}
                  </div>
                  {getStatusBadge(election.status)}
                </div>

                <div className="mt-4 space-y-2">
                  <div className="flex items-center text-sm text-gray-500">
                    <Calendar className="h-4 w-4 mr-2" />
                    <span>
                      {formatDate(election.start_date)} - {formatDate(election.end_date)}
                    </span>
                  </div>
                  
                  <div className="flex items-center text-sm text-gray-500">
                    <Users className="h-4 w-4 mr-2" />
                    <span>
                      {election.candidates?.length || 0} candidatos
                    </span>
                  </div>
                  
                  <div className="flex items-center text-sm text-gray-500">
                    <Vote className="h-4 w-4 mr-2" />
                    <span>
                      {election.total_votes || 0} votos
                    </span>
                  </div>
                </div>

                <div className="mt-6 flex space-x-3">
                  <Link
                    to={`/elections/${election.id}`}
                    className="btn-secondary flex-1 text-center"
                  >
                    <Eye className="h-4 w-4 mr-2" />
                    Ver Detalhes
                  </Link>
                </div>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default ElectionsPage;
