// FORTIS Frontend - Página de Auditoria

import React from 'react';
import { Shield, FileText, CheckCircle, AlertTriangle } from 'lucide-react';

const AuditPage: React.FC = () => {
  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Auditoria</h1>
          <p className="mt-1 text-sm text-gray-500">Sistema de auditoria e verificação</p>
        </div>
      </div>

      <div className="card">
        <div className="card-body">
          <div className="text-center py-12">
            <Shield className="mx-auto h-12 w-12 text-gray-400" />
            <h3 className="mt-2 text-sm font-medium text-gray-900">
              Página em desenvolvimento
            </h3>
            <p className="mt-1 text-sm text-gray-500">
              A funcionalidade de auditoria será implementada em breve.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default AuditPage;
