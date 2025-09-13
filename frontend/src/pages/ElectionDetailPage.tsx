// FORTIS Frontend - Detalhes da Eleição

import React from 'react';
import { useParams } from 'react-router-dom';
import { ArrowLeft, Vote, Users, Calendar, Shield } from 'lucide-react';
import { Link } from 'react-router-dom';

const ElectionDetailPage: React.FC = () => {
  const { id } = useParams<{ id: string }>();

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center space-x-4">
        <Link to="/elections" className="btn-secondary">
          <ArrowLeft className="h-4 w-4 mr-2" />
          Voltar
        </Link>
        <div>
          <h1 className="text-2xl font-bold text-gray-900">
            Eleição Municipal 2025
          </h1>
          <p className="mt-1 text-sm text-gray-500">
            Detalhes e estatísticas da eleição
          </p>
        </div>
      </div>

      {/* Stats */}
      <div className="grid grid-cols-1 gap-5 sm:grid-cols-4">
        <div className="card">
          <div className="card-body text-center">
            <Vote className="h-8 w-8 text-primary-600 mx-auto" />
            <p className="mt-2 text-2xl font-bold text-gray-900">1,234,567</p>
            <p className="text-sm text-gray-500">Total de Votos</p>
          </div>
        </div>
        <div className="card">
          <div className="card-body text-center">
            <Users className="h-8 w-8 text-primary-600 mx-auto" />
            <p className="mt-2 text-2xl font-bold text-gray-900">12</p>
            <p className="text-sm text-gray-500">Candidatos</p>
          </div>
        </div>
        <div className="card">
          <div className="card-body text-center">
            <Calendar className="h-8 w-8 text-primary-600 mx-auto" />
            <p className="mt-2 text-2xl font-bold text-gray-900">85.2%</p>
            <p className="text-sm text-gray-500">Participação</p>
          </div>
        </div>
        <div className="card">
          <div className="card-body text-center">
            <Shield className="h-8 w-8 text-primary-600 mx-auto" />
            <p className="mt-2 text-2xl font-bold text-gray-900">100%</p>
            <p className="text-sm text-gray-500">Auditado</p>
          </div>
        </div>
      </div>

      {/* Content */}
      <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
        <div className="card">
          <div className="card-header">
            <h3 className="text-lg font-medium text-gray-900">
              Informações da Eleição
            </h3>
          </div>
          <div className="card-body">
            <dl className="space-y-4">
              <div>
                <dt className="text-sm font-medium text-gray-500">Título</dt>
                <dd className="mt-1 text-sm text-gray-900">Eleição Municipal 2025</dd>
              </div>
              <div>
                <dt className="text-sm font-medium text-gray-500">Período</dt>
                <dd className="mt-1 text-sm text-gray-900">
                  15/10/2025 08:00 - 15/10/2025 17:00
                </dd>
              </div>
              <div>
                <dt className="text-sm font-medium text-gray-500">Status</dt>
                <dd className="mt-1">
                  <span className="badge-success">Ativa</span>
                </dd>
              </div>
            </dl>
          </div>
        </div>

        <div className="card">
          <div className="card-header">
            <h3 className="text-lg font-medium text-gray-900">
              Resultados
            </h3>
          </div>
          <div className="card-body">
            <div className="space-y-4">
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-900">João Silva (PT)</span>
                <span className="text-sm font-medium text-gray-900">45.2%</span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-900">Maria Santos (PSDB)</span>
                <span className="text-sm font-medium text-gray-900">38.7%</span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-900">Pedro Costa (MDB)</span>
                <span className="text-sm font-medium text-gray-900">16.1%</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ElectionDetailPage;
