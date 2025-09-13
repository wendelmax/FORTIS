// FORTIS Frontend - Página de Configurações

import React from 'react';
import { Settings, User, Shield, Database } from 'lucide-react';

const SettingsPage: React.FC = () => {
  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-gray-900">Configurações</h1>
        <p className="mt-1 text-sm text-gray-500">Configurações do sistema</p>
      </div>

      <div className="card">
        <div className="card-body">
          <div className="text-center py-12">
            <Settings className="mx-auto h-12 w-12 text-gray-400" />
            <h3 className="mt-2 text-sm font-medium text-gray-900">
              Página em desenvolvimento
            </h3>
            <p className="mt-1 text-sm text-gray-500">
              As configurações do sistema serão implementadas em breve.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default SettingsPage;
