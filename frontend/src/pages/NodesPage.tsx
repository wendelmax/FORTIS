// FORTIS Frontend - Página de Nós

import React from 'react';
import { Plus, Network, CheckCircle, XCircle, Clock } from 'lucide-react';

const NodesPage: React.FC = () => {
  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Nós Distribuídos</h1>
          <p className="mt-1 text-sm text-gray-500">Gerencie a rede distribuída do FORTIS</p>
        </div>
        <button className="btn-primary">
          <Plus className="h-5 w-5 mr-2" />
          Adicionar Nó
        </button>
      </div>

      <div className="card">
        <div className="card-body">
          <div className="text-center py-12">
            <Network className="mx-auto h-12 w-12 text-gray-400" />
            <h3 className="mt-2 text-sm font-medium text-gray-900">
              Página em desenvolvimento
            </h3>
            <p className="mt-1 text-sm text-gray-500">
              A funcionalidade de gerenciamento de nós será implementada em breve.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default NodesPage;
