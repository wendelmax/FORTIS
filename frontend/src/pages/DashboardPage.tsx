// FORTIS Frontend - Dashboard

import React from 'react';
import { 
  Vote, 
  Users, 
  Network, 
  Shield, 
  TrendingUp, 
  Clock,
  CheckCircle,
  AlertCircle
} from 'lucide-react';

const DashboardPage: React.FC = () => {
  const stats = [
    {
      name: 'Eleições Ativas',
      value: '3',
      change: '+1',
      changeType: 'positive',
      icon: Vote,
    },
    {
      name: 'Total de Votos',
      value: '1,234,567',
      change: '+12.5%',
      changeType: 'positive',
      icon: TrendingUp,
    },
    {
      name: 'Eleitores Cadastrados',
      value: '45,678,901',
      change: '+2.1%',
      changeType: 'positive',
      icon: Users,
    },
    {
      name: 'Nós Online',
      value: '27/27',
      change: '100%',
      changeType: 'positive',
      icon: Network,
    },
  ];

  const recentActivities = [
    {
      id: 1,
      type: 'election',
      title: 'Eleição Municipal 2025',
      description: 'Nova eleição criada e ativada',
      time: '2 horas atrás',
      status: 'active',
    },
    {
      id: 2,
      type: 'vote',
      title: 'Voto registrado',
      description: 'Voto verificado e adicionado ao blockchain',
      time: '5 minutos atrás',
      status: 'success',
    },
    {
      id: 3,
      type: 'node',
      title: 'Nó SP-01',
      description: 'Nó de São Paulo sincronizado com sucesso',
      time: '1 hora atrás',
      status: 'success',
    },
    {
      id: 4,
      type: 'audit',
      title: 'Auditoria realizada',
      description: 'Auditoria de integridade concluída',
      time: '3 horas atrás',
      status: 'success',
    },
  ];

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'active':
        return <Clock className="h-4 w-4 text-blue-500" />;
      case 'success':
        return <CheckCircle className="h-4 w-4 text-green-500" />;
      case 'error':
        return <AlertCircle className="h-4 w-4 text-red-500" />;
      default:
        return <Clock className="h-4 w-4 text-gray-500" />;
    }
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-2xl font-bold text-gray-900">Dashboard</h1>
        <p className="mt-1 text-sm text-gray-500">
          Visão geral do sistema FORTIS
        </p>
      </div>

      {/* Stats Grid */}
      <div className="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4">
        {stats.map((stat) => (
          <div key={stat.name} className="card">
            <div className="card-body">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <stat.icon className="h-8 w-8 text-primary-600" />
                </div>
                <div className="ml-5 w-0 flex-1">
                  <dl>
                    <dt className="text-sm font-medium text-gray-500 truncate">
                      {stat.name}
                    </dt>
                    <dd className="flex items-baseline">
                      <div className="text-2xl font-semibold text-gray-900">
                        {stat.value}
                      </div>
                      <div className={`ml-2 flex items-baseline text-sm font-semibold ${
                        stat.changeType === 'positive' ? 'text-green-600' : 'text-red-600'
                      }`}>
                        {stat.change}
                      </div>
                    </dd>
                  </dl>
                </div>
              </div>
            </div>
          </div>
        ))}
      </div>

      {/* Main Content Grid */}
      <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
        {/* Recent Activities */}
        <div className="card">
          <div className="card-header">
            <h3 className="text-lg font-medium text-gray-900">
              Atividades Recentes
            </h3>
          </div>
          <div className="card-body">
            <div className="flow-root">
              <ul className="-mb-8">
                {recentActivities.map((activity, activityIdx) => (
                  <li key={activity.id}>
                    <div className="relative pb-8">
                      {activityIdx !== recentActivities.length - 1 ? (
                        <span
                          className="absolute top-4 left-4 -ml-px h-full w-0.5 bg-gray-200"
                          aria-hidden="true"
                        />
                      ) : null}
                      <div className="relative flex space-x-3">
                        <div>
                          <span className="h-8 w-8 rounded-full bg-gray-100 flex items-center justify-center ring-8 ring-white">
                            {getStatusIcon(activity.status)}
                          </span>
                        </div>
                        <div className="min-w-0 flex-1 pt-1.5 flex justify-between space-x-4">
                          <div>
                            <p className="text-sm text-gray-900">
                              {activity.title}
                            </p>
                            <p className="text-sm text-gray-500">
                              {activity.description}
                            </p>
                          </div>
                          <div className="text-right text-sm whitespace-nowrap text-gray-500">
                            {activity.time}
                          </div>
                        </div>
                      </div>
                    </div>
                  </li>
                ))}
              </ul>
            </div>
          </div>
        </div>

        {/* System Status */}
        <div className="card">
          <div className="card-header">
            <h3 className="text-lg font-medium text-gray-900">
              Status do Sistema
            </h3>
          </div>
          <div className="card-body">
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <div className="flex items-center">
                  <CheckCircle className="h-5 w-5 text-green-500" />
                  <span className="ml-2 text-sm font-medium text-gray-900">
                    Backend
                  </span>
                </div>
                <span className="badge-success">Online</span>
              </div>
              
              <div className="flex items-center justify-between">
                <div className="flex items-center">
                  <CheckCircle className="h-5 w-5 text-green-500" />
                  <span className="ml-2 text-sm font-medium text-gray-900">
                    Blockchain
                  </span>
                </div>
                <span className="badge-success">Sincronizado</span>
              </div>
              
              <div className="flex items-center justify-between">
                <div className="flex items-center">
                  <CheckCircle className="h-5 w-5 text-green-500" />
                  <span className="ml-2 text-sm font-medium text-gray-900">
                    TSE Integration
                  </span>
                </div>
                <span className="badge-success">Conectado</span>
              </div>
              
              <div className="flex items-center justify-between">
                <div className="flex items-center">
                  <CheckCircle className="h-5 w-5 text-green-500" />
                  <span className="ml-2 text-sm font-medium text-gray-900">
                    Nós Distribuídos
                  </span>
                </div>
                <span className="badge-success">27/27 Online</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Quick Actions */}
      <div className="card">
        <div className="card-header">
          <h3 className="text-lg font-medium text-gray-900">
            Ações Rápidas
          </h3>
        </div>
        <div className="card-body">
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
            <button className="btn-primary">
              <Vote className="h-5 w-5 mr-2" />
              Nova Eleição
            </button>
            <button className="btn-secondary">
              <Users className="h-5 w-5 mr-2" />
              Gerenciar Candidatos
            </button>
            <button className="btn-secondary">
              <Network className="h-5 w-5 mr-2" />
              Ver Nós
            </button>
            <button className="btn-secondary">
              <Shield className="h-5 w-5 mr-2" />
              Auditoria
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default DashboardPage;
