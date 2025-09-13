# FORTIS - Frontend Administrativo e Gestão
## Frontend Developer Perspective

### 🎯 **Visão Geral do Frontend**

O FORTIS implementa um sistema administrativo completo usando React + TypeScript, fornecendo dashboards executivos, gestão de eleições, configuração de nós distribuídos, aprovações ministeriais e data lake para análise e transparência total do sistema eleitoral brasileiro.

---

## 🛠️ **Stack Tecnológico Frontend**

### **Framework Principal: React 18 + TypeScript**
```json
{
  "dependencies": {
    "react": "18.2.0",
    "typescript": "5.0.0",
    "vite": "4.4.0",
    "tailwindcss": "3.3.0",
    "zustand": "4.4.0",
    "react-query": "3.39.0",
    "ethers": "6.7.0",
    "wagmi": "1.4.0",
    "framer-motion": "10.16.0"
  }
}
```

### **Por que React + TypeScript?**
- **Type Safety**: Prevenção de erros em tempo de compilação
- **Performance**: React 18 com Concurrent Features
- **Ecosistema**: Bibliotecas maduras e comunidade ativa
- **Manutenibilidade**: Código limpo e escalável

---

## 🎨 **Design System e Componentes**

### **Estrutura de Componentes Administrativos**
```
src/
├── components/
│   ├── ui/                    # Componentes base
│   │   ├── Button.tsx
│   │   ├── Input.tsx
│   │   ├── Modal.tsx
│   │   └── Card.tsx
│   ├── dashboard/             # Dashboards executivos
│   │   ├── ExecutiveDashboard.tsx
│   │   ├── ElectionOverview.tsx
│   │   ├── SystemHealth.tsx
│   │   └── RealTimeMetrics.tsx
│   ├── election-management/   # Gestão de eleições
│   │   ├── ElectionConfig.tsx
│   │   ├── CandidateManagement.tsx
│   │   ├── ElectionSchedule.tsx
│   │   └── ElectionKeys.tsx
│   ├── node-management/       # Gestão de nós distribuídos
│   │   ├── NodeRegistry.tsx
│   │   ├── NodeStatus.tsx
│   │   ├── NodeConfiguration.tsx
│   │   └── NetworkTopology.tsx
│   ├── ministerial-approval/  # Aprovações ministeriais
│   │   ├── ApprovalWorkflow.tsx
│   │   ├── MinisterDashboard.tsx
│   │   ├── DigitalSignature.tsx
│   │   └── GovBrIntegration.tsx
│   ├── data-lake/            # Data lake e analytics
│   │   ├── DataExplorer.tsx
│   │   ├── QueryBuilder.tsx
│   │   ├── ReportGenerator.tsx
│   │   └── DataVisualization.tsx
│   └── security/             # Gestão de segurança
│       ├── KeyManagement.tsx
│       ├── AccessControl.tsx
│       ├── AuditLogs.tsx
│       └── SecurityMonitoring.tsx
```

### **Dashboard Executivo Principal**
```typescript
// ExecutiveDashboard.tsx
import React, { useState, useEffect } from 'react';
import { useElectionData } from '../../hooks/useElectionData';
import { useSystemMetrics } from '../../hooks/useSystemMetrics';
import { ElectionOverview } from './ElectionOverview';
import { SystemHealth } from './SystemHealth';
import { RealTimeMetrics } from './RealTimeMetrics';

interface ExecutiveDashboardProps {
  userRole: 'minister' | 'admin' | 'auditor';
}

export const ExecutiveDashboard: React.FC<ExecutiveDashboardProps> = ({ userRole }) => {
  const [activeTab, setActiveTab] = useState<'overview' | 'elections' | 'nodes' | 'security'>('overview');
  const [electionData, setElectionData] = useState(null);
  const [systemMetrics, setSystemMetrics] = useState(null);
  
  const { elections, loading: electionsLoading } = useElectionData();
  const { metrics, loading: metricsLoading } = useSystemMetrics();
  
  const handleElectionCreate = async (electionConfig: ElectionConfig) => {
    try {
      // Criar nova eleição
      const newElection = await createElection(electionConfig);
      setElectionData(prev => [...prev, newElection]);
    } catch (error) {
      console.error('Failed to create election:', error);
    }
  };
  
  const handleNodeConfiguration = async (nodeConfig: NodeConfig) => {
    try {
      // Configurar nó distribuído
      await configureNode(nodeConfig);
    } catch (error) {
      console.error('Failed to configure node:', error);
    }
  };
  
  return (
    <div className="min-h-screen bg-gray-50">
      <div className="flex">
        {/* Sidebar Navigation */}
        <nav className="w-64 bg-white shadow-lg">
          <div className="p-6">
            <h1 className="text-2xl font-bold text-gray-900">FORTIS Admin</h1>
            <p className="text-sm text-gray-600">Sistema de Gestão Eleitoral</p>
          </div>
          
          <div className="mt-6">
            <button
              onClick={() => setActiveTab('overview')}
              className={`w-full text-left px-6 py-3 ${activeTab === 'overview' ? 'bg-blue-50 text-blue-700' : 'text-gray-700 hover:bg-gray-50'}`}
            >
              📊 Visão Geral
            </button>
            <button
              onClick={() => setActiveTab('elections')}
              className={`w-full text-left px-6 py-3 ${activeTab === 'elections' ? 'bg-blue-50 text-blue-700' : 'text-gray-700 hover:bg-gray-50'}`}
            >
              🗳️ Eleições
            </button>
            <button
              onClick={() => setActiveTab('nodes')}
              className={`w-full text-left px-6 py-3 ${activeTab === 'nodes' ? 'bg-blue-50 text-blue-700' : 'text-gray-700 hover:bg-gray-50'}`}
            >
              🌐 Nós Distribuídos
            </button>
            <button
              onClick={() => setActiveTab('security')}
              className={`w-full text-left px-6 py-3 ${activeTab === 'security' ? 'bg-blue-50 text-blue-700' : 'text-gray-700 hover:bg-gray-50'}`}
            >
              🔒 Segurança
            </button>
          </div>
        </nav>
        
        {/* Main Content */}
        <main className="flex-1 p-8">
          {activeTab === 'overview' && (
            <div className="space-y-6">
              <ElectionOverview elections={elections} />
              <SystemHealth metrics={metrics} />
              <RealTimeMetrics />
            </div>
          )}
          
          {activeTab === 'elections' && (
            <ElectionManagement onElectionCreate={handleElectionCreate} />
          )}
          
          {activeTab === 'nodes' && (
            <NodeManagement onNodeConfigure={handleNodeConfiguration} />
          )}
          
          {activeTab === 'security' && (
            <SecurityManagement />
          )}
        </main>
      </div>
    </div>
  );
};
```

### **Gestão de Eleições**
```typescript
// ElectionManagement.tsx
import React, { useState } from 'react';
import { ElectionConfig } from './ElectionConfig';
import { CandidateManagement } from './CandidateManagement';
import { ElectionKeys } from './ElectionKeys';

export const ElectionManagement: React.FC = () => {
  const [activeElection, setActiveElection] = useState(null);
  const [step, setStep] = useState<'config' | 'candidates' | 'keys' | 'schedule'>('config');
  
  return (
    <div className="space-y-6">
      <div className="bg-white rounded-lg shadow p-6">
        <h2 className="text-2xl font-bold mb-4">Gestão de Eleições</h2>
        
        {/* Configuração da Eleição */}
        {step === 'config' && (
          <ElectionConfig
            onNext={() => setStep('candidates')}
            onSave={handleElectionSave}
          />
        )}
        
        {/* Gestão de Candidatos */}
        {step === 'candidates' && (
          <CandidateManagement
            electionId={activeElection?.id}
            onNext={() => setStep('keys')}
            onBack={() => setStep('config')}
          />
        )}
        
        {/* Chaves Criptográficas */}
        {step === 'keys' && (
          <ElectionKeys
            electionId={activeElection?.id}
            onNext={() => setStep('schedule')}
            onBack={() => setStep('candidates')}
          />
        )}
        
        {/* Agendamento */}
        {step === 'schedule' && (
          <ElectionSchedule
            electionId={activeElection?.id}
            onComplete={handleElectionComplete}
            onBack={() => setStep('keys')}
          />
        )}
      </div>
    </div>
  );
};
```

### **Gestão de Nós Distribuídos**
```typescript
// NodeManagement.tsx
import React, { useState, useEffect } from 'react';
import { NodeRegistry } from './NodeRegistry';
import { NetworkTopology } from './NetworkTopology';
import { NodeConfiguration } from './NodeConfiguration';

export const NodeManagement: React.FC = () => {
  const [nodes, setNodes] = useState([]);
  const [selectedNode, setSelectedNode] = useState(null);
  const [view, setView] = useState<'list' | 'topology' | 'config'>('list');
  
  const handleNodeAdd = async (nodeData: NodeData) => {
    try {
      // Adicionar novo nó à rede
      const newNode = await addNode(nodeData);
      setNodes(prev => [...prev, newNode]);
    } catch (error) {
      console.error('Failed to add node:', error);
    }
  };
  
  const handleNodeConfigure = async (nodeId: string, config: NodeConfig) => {
    try {
      // Configurar nó existente
      await configureNode(nodeId, config);
      setNodes(prev => prev.map(node => 
        node.id === nodeId ? { ...node, ...config } : node
      ));
    } catch (error) {
      console.error('Failed to configure node:', error);
    }
  };
  
  return (
    <div className="space-y-6">
      <div className="bg-white rounded-lg shadow p-6">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-2xl font-bold">Nós Distribuídos</h2>
          <div className="flex space-x-2">
            <button
              onClick={() => setView('list')}
              className={`px-4 py-2 rounded ${view === 'list' ? 'bg-blue-500 text-white' : 'bg-gray-200'}`}
            >
              Lista
            </button>
            <button
              onClick={() => setView('topology')}
              className={`px-4 py-2 rounded ${view === 'topology' ? 'bg-blue-500 text-white' : 'bg-gray-200'}`}
            >
              Topologia
            </button>
            <button
              onClick={() => setView('config')}
              className={`px-4 py-2 rounded ${view === 'config' ? 'bg-blue-500 text-white' : 'bg-gray-200'}`}
            >
              Configuração
            </button>
          </div>
        </div>
        
        {view === 'list' && (
          <NodeRegistry
            nodes={nodes}
            onNodeSelect={setSelectedNode}
            onNodeAdd={handleNodeAdd}
          />
        )}
        
        {view === 'topology' && (
          <NetworkTopology nodes={nodes} />
        )}
        
        {view === 'config' && selectedNode && (
          <NodeConfiguration
            node={selectedNode}
            onConfigure={handleNodeConfigure}
          />
        )}
      </div>
    </div>
  );
};
```

### **Aprovações Ministeriais e Gov.br**
```typescript
// MinisterialApproval.tsx
import React, { useState } from 'react';
import { GovBrIntegration } from './GovBrIntegration';
import { DigitalSignature } from './DigitalSignature';
import { ApprovalWorkflow } from './ApprovalWorkflow';

export const MinisterialApproval: React.FC = () => {
  const [pendingApprovals, setPendingApprovals] = useState([]);
  const [ministerAuth, setMinisterAuth] = useState(null);
  
  const handleGovBrAuth = async (cpf: string) => {
    try {
      // Integração com gov.br para autenticação
      const authResult = await authenticateWithGovBr(cpf);
      setMinisterAuth(authResult);
    } catch (error) {
      console.error('Gov.br authentication failed:', error);
    }
  };
  
  const handleApproval = async (approvalId: string, decision: 'approve' | 'reject') => {
    try {
      // Processar aprovação com assinatura digital
      await processApproval(approvalId, decision, ministerAuth);
      setPendingApprovals(prev => prev.filter(a => a.id !== approvalId));
    } catch (error) {
      console.error('Approval processing failed:', error);
    }
  };
  
  return (
    <div className="space-y-6">
      {/* Autenticação Gov.br */}
      <div className="bg-white rounded-lg shadow p-6">
        <h3 className="text-lg font-semibold mb-4">Autenticação Ministerial</h3>
        <GovBrIntegration onAuth={handleGovBrAuth} />
      </div>
      
      {/* Workflow de Aprovações */}
      {ministerAuth && (
        <div className="bg-white rounded-lg shadow p-6">
          <h3 className="text-lg font-semibold mb-4">Aprovações Pendentes</h3>
          <ApprovalWorkflow
            approvals={pendingApprovals}
            onApproval={handleApproval}
            ministerAuth={ministerAuth}
          />
        </div>
      )}
      
      {/* Assinatura Digital */}
      {ministerAuth && (
        <div className="bg-white rounded-lg shadow p-6">
          <h3 className="text-lg font-semibold mb-4">Assinatura Digital</h3>
          <DigitalSignature
            ministerAuth={ministerAuth}
            onSign={handleDigitalSignature}
          />
        </div>
      )}
    </div>
  );
};
```

### **Data Lake e Analytics**
```typescript
// DataLakeExplorer.tsx
import React, { useState } from 'react';
import { DataExplorer } from './DataExplorer';
import { QueryBuilder } from './QueryBuilder';
import { ReportGenerator } from './ReportGenerator';

export const DataLakeExplorer: React.FC = () => {
  const [activeDataset, setActiveDataset] = useState(null);
  const [query, setQuery] = useState('');
  const [results, setResults] = useState(null);
  
  const handleQueryExecute = async (query: string) => {
    try {
      // Executar query no data lake
      const queryResults = await executeDataLakeQuery(query);
      setResults(queryResults);
    } catch (error) {
      console.error('Query execution failed:', error);
    }
  };
  
  const handleReportGenerate = async (reportConfig: ReportConfig) => {
    try {
      // Gerar relatório personalizado
      const report = await generateReport(reportConfig);
      downloadReport(report);
    } catch (error) {
      console.error('Report generation failed:', error);
    }
  };
  
  return (
    <div className="space-y-6">
      {/* Explorador de Dados */}
      <div className="bg-white rounded-lg shadow p-6">
        <h3 className="text-lg font-semibold mb-4">Explorador de Dados</h3>
        <DataExplorer
          onDatasetSelect={setActiveDataset}
          selectedDataset={activeDataset}
        />
      </div>
      
      {/* Query Builder */}
      <div className="bg-white rounded-lg shadow p-6">
        <h3 className="text-lg font-semibold mb-4">Query Builder</h3>
        <QueryBuilder
          dataset={activeDataset}
          onQueryExecute={handleQueryExecute}
          results={results}
        />
      </div>
      
      {/* Gerador de Relatórios */}
      <div className="bg-white rounded-lg shadow p-6">
        <h3 className="text-lg font-semibold mb-4">Relatórios</h3>
        <ReportGenerator
          onReportGenerate={handleReportGenerate}
          availableDatasets={availableDatasets}
        />
      </div>
    </div>
  );
};
```

---

## 📱 **Responsividade e Mobile-First**

### **Breakpoints Tailwind**
```typescript
// tailwind.config.js
export default {
  theme: {
    extend: {
      screens: {
        'xs': '475px',
        'sm': '640px',
        'md': '768px',
        'lg': '1024px',
        'xl': '1280px',
        '2xl': '1536px',
      },
    },
  },
}
```

### **Componente Responsivo**
```typescript
// CandidateCard.tsx
export const CandidateCard: React.FC<CandidateCardProps> = ({ candidate, onSelect }) => {
  return (
    <div className="
      w-full sm:w-1/2 lg:w-1/3 xl:w-1/4
      p-4 sm:p-6
      bg-white rounded-lg shadow-md
      hover:shadow-lg transition-shadow
      cursor-pointer
    ">
      <div className="flex flex-col sm:flex-row items-center space-y-4 sm:space-y-0 sm:space-x-4">
        <img 
          src={candidate.photo} 
          alt={candidate.name}
          className="w-16 h-16 sm:w-20 sm:h-20 rounded-full object-cover"
        />
        <div className="text-center sm:text-left">
          <h3 className="text-lg font-semibold text-gray-900">
            {candidate.name}
          </h3>
          <p className="text-sm text-gray-600">
            {candidate.party}
          </p>
          <p className="text-2xl font-bold text-blue-600">
            {candidate.number}
          </p>
        </div>
      </div>
    </div>
  );
};
```

---

## ♿ **Acessibilidade Universal**

### **Implementação WCAG 2.1**
```typescript
// AcessibleButton.tsx
export const AcessibleButton: React.FC<ButtonProps> = ({ 
  children, 
  onClick, 
  disabled,
  ariaLabel,
  ...props 
}) => {
  return (
    <button
      onClick={onClick}
      disabled={disabled}
      aria-label={ariaLabel}
      className={`
        px-4 py-2 rounded-md font-medium
        focus:outline-none focus:ring-2 focus:ring-blue-500
        disabled:opacity-50 disabled:cursor-not-allowed
        transition-colors duration-200
        ${disabled ? 'bg-gray-300' : 'bg-blue-600 hover:bg-blue-700 text-white'}
      `}
      {...props}
    >
      {children}
    </button>
  );
};
```

### **Suporte a Leitores de Tela**
```typescript
// VotingStep.tsx
export const VotingStep: React.FC<VotingStepProps> = ({ step, children }) => {
  return (
    <div 
      role="main" 
      aria-labelledby="voting-step-title"
      className="min-h-screen bg-gray-50"
    >
      <h1 
        id="voting-step-title" 
        className="sr-only"
      >
        {step === 'auth' && 'Autenticação Biométrica'}
        {step === 'select' && 'Seleção de Candidato'}
        {step === 'confirm' && 'Confirmação de Voto'}
        {step === 'receipt' && 'Comprovante de Voto'}
      </h1>
      
      <div className="container mx-auto px-4 py-8">
        {children}
      </div>
    </div>
  );
};
```

---

## 🚀 **Performance e Otimização**

### **Lazy Loading e Code Splitting**
```typescript
// App.tsx
import { lazy, Suspense } from 'react';

const VotingInterface = lazy(() => import('./components/voting/VotingInterface'));
const PublicAudit = lazy(() => import('./components/audit/PublicAudit'));
const AdminDashboard = lazy(() => import('./components/admin/AdminDashboard'));

export const App: React.FC = () => {
  return (
    <Router>
      <Suspense fallback={<LoadingSpinner />}>
        <Routes>
          <Route path="/vote" element={<VotingInterface />} />
          <Route path="/audit" element={<PublicAudit />} />
          <Route path="/admin" element={<AdminDashboard />} />
        </Routes>
      </Suspense>
    </Router>
  );
};
```

### **Otimização de Re-renders**
```typescript
// CandidateList.tsx
import React, { memo, useMemo } from 'react';

interface CandidateListProps {
  candidates: Candidate[];
  onSelect: (candidateId: string) => void;
}

export const CandidateList = memo<CandidateListProps>(({ candidates, onSelect }) => {
  const sortedCandidates = useMemo(() => 
    candidates.sort((a, b) => a.number.localeCompare(b.number)),
    [candidates]
  );

  return (
    <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
      {sortedCandidates.map(candidate => (
        <CandidateCard
          key={candidate.id}
          candidate={candidate}
          onSelect={onSelect}
        />
      ))}
    </div>
  );
});
```

---

## 🔗 **Integração com Blockchain e Urnas**

### **Gestão de Chaves Criptográficas**
```typescript
// useElectionKeys.ts
import { useMutation, useQuery, useQueryClient } from 'react-query';

export const useElectionKeys = () => {
  const queryClient = useQueryClient();
  
  const generateElectionKeys = useMutation(
    async (electionId: string) => {
      // Gerar chaves criptográficas para a eleição
      const response = await fetch('/api/v1/elections/keys/generate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ electionId })
      });
      
      if (!response.ok) throw new Error('Failed to generate keys');
      return response.json();
    },
    {
      onSuccess: () => {
        queryClient.invalidateQueries(['election-keys']);
      },
    }
  );
  
  const distributeKeysToUrnas = useMutation(
    async ({ electionId, urnaIds }: { electionId: string, urnaIds: string[] }) => {
      // Distribuir chaves para as urnas
      const response = await fetch('/api/v1/elections/keys/distribute', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ electionId, urnaIds })
      });
      
      if (!response.ok) throw new Error('Failed to distribute keys');
      return response.json();
    }
  );
  
  return {
    generateElectionKeys: generateElectionKeys.mutateAsync,
    distributeKeysToUrnas: distributeKeysToUrnas.mutateAsync,
    isGenerating: generateElectionKeys.isLoading,
    isDistributing: distributeKeysToUrnas.isLoading,
  };
};
```

### **Monitoramento de Urnas**
```typescript
// useUrnaMonitoring.ts
import { useQuery, useMutation } from 'react-query';

export const useUrnaMonitoring = () => {
  const { data: urnas, isLoading } = useQuery(
    ['urnas'],
    async () => {
      const response = await fetch('/api/v1/urnas');
      if (!response.ok) throw new Error('Failed to fetch urnas');
      return response.json();
    },
    { refetchInterval: 5000 } // Atualizar a cada 5 segundos
  );
  
  const syncUrna = useMutation(
    async (urnaId: string) => {
      // Sincronizar urna específica
      const response = await fetch(`/api/v1/urnas/${urnaId}/sync`, {
        method: 'POST'
      });
      
      if (!response.ok) throw new Error('Failed to sync urna');
      return response.json();
    }
  );
  
  const getUrnaStatus = useQuery(
    ['urna-status'],
    async () => {
      const response = await fetch('/api/v1/urnas/status');
      if (!response.ok) throw new Error('Failed to fetch urna status');
      return response.json();
    },
    { refetchInterval: 2000 }
  );
  
  return {
    urnas,
    isLoading,
    syncUrna: syncUrna.mutateAsync,
    urnaStatus: getUrnaStatus.data,
    isSyncing: syncUrna.isLoading,
  };
};
```

### **Comunicação Segura com Urnas**
```typescript
// useSecureCommunication.ts
import { useMutation } from 'react-query';

export const useSecureCommunication = () => {
  const sendSecureCommand = useMutation(
    async ({ urnaId, command, data }: { urnaId: string, command: string, data: any }) => {
      // Enviar comando seguro para urna
      const response = await fetch(`/api/v1/urnas/${urnaId}/command`, {
        method: 'POST',
        headers: { 
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${getAuthToken()}`
        },
        body: JSON.stringify({ command, data })
      });
      
      if (!response.ok) throw new Error('Failed to send command');
      return response.json();
    }
  );
  
  const verifyUrnaIntegrity = useMutation(
    async (urnaId: string) => {
      // Verificar integridade da urna
      const response = await fetch(`/api/v1/urnas/${urnaId}/verify`, {
        method: 'POST'
      });
      
      if (!response.ok) throw new Error('Failed to verify urna');
      return response.json();
    }
  );
  
  return {
    sendSecureCommand: sendSecureCommand.mutateAsync,
    verifyUrnaIntegrity: verifyUrnaIntegrity.mutateAsync,
    isSending: sendSecureCommand.isLoading,
    isVerifying: verifyUrnaIntegrity.isLoading,
  };
};
```

---

## 🎭 **Animações e Micro-interações**

### **Framer Motion para Animações**
```typescript
// AnimatedCard.tsx
import { motion } from 'framer-motion';

export const AnimatedCard: React.FC<CardProps> = ({ children, onClick }) => {
  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -20 }}
      whileHover={{ scale: 1.02 }}
      whileTap={{ scale: 0.98 }}
      transition={{ duration: 0.2 }}
      onClick={onClick}
      className="bg-white rounded-lg shadow-md p-6 cursor-pointer"
    >
      {children}
    </motion.div>
  );
};
```

---

## 📊 **Métricas de Performance**

### **Core Web Vitals**
- **LCP (Largest Contentful Paint)**: < 2.5s
- **FID (First Input Delay)**: < 100ms
- **CLS (Cumulative Layout Shift)**: < 0.1
- **Bundle Size**: < 200KB gzipped
- **Time to Interactive**: < 3.9s

### **Monitoramento de Performance**
```typescript
// performance.ts
export const trackPerformance = () => {
  if ('web-vitals' in window) {
    import('web-vitals').then(({ getCLS, getFID, getFCP, getLCP, getTTFB }) => {
      getCLS(console.log);
      getFID(console.log);
      getFCP(console.log);
      getLCP(console.log);
      getTTFB(console.log);
    });
  }
};
```

---

## 🧪 **Testes Frontend**

### **Testes com Testing Library**
```typescript
// VotingInterface.test.tsx
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { VotingInterface } from './VotingInterface';

describe('VotingInterface', () => {
  it('should render authentication step initially', () => {
    render(<VotingInterface electionId="test-election" />);
    
    expect(screen.getByText('Autenticação Biométrica')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /iniciar autenticação/i })).toBeInTheDocument();
  });
  
  it('should proceed to candidate selection after authentication', async () => {
    render(<VotingInterface electionId="test-election" />);
    
    const authButton = screen.getByRole('button', { name: /iniciar autenticação/i });
    fireEvent.click(authButton);
    
    await waitFor(() => {
      expect(screen.getByText('Selecione seu candidato')).toBeInTheDocument();
    });
  });
});
```

---

## 🎯 **Próximos Passos**

### **Fase 1: Dashboard Administrativo (2 meses)**
- [ ] Design System para admin
- [ ] Dashboard executivo principal
- [ ] Gestão de eleições básica
- [ ] Autenticação gov.br
- [ ] Testes unitários

### **Fase 2: Gestão Avançada (2 meses)**
- [ ] Gestão de nós distribuídos
- [ ] Aprovações ministeriais
- [ ] Data lake e analytics
- [ ] Integração com urnas
- [ ] Testes de integração

### **Fase 3: Produção (2 meses)**
- [ ] Performance optimization
- [ ] Segurança avançada
- [ ] Testes E2E
- [ ] Deploy e monitoramento
- [ ] Treinamento de usuários

---

## 📱 **App Mobile Futuro (Integração Gov.br)**

### **Visão Geral do App Mobile**
O FORTIS Mobile será um aplicativo complementar que permitirá votação via smartphone, integrado com o gov.br para autenticação e validação de identidade, mantendo a mesma segurança e transparência do sistema principal.

### **Arquitetura Mobile**
```typescript
// mobile/FortisMobile.tsx
import React from 'react';
import { NavigationContainer } from '@react-navigation/native';
import { createStackNavigator } from '@react-navigation/stack';
import { GovBrAuth } from './screens/GovBrAuth';
import { VotingInterface } from './screens/VotingInterface';
import { VoteVerification } from './screens/VoteVerification';

const Stack = createStackNavigator();

export const FortisMobile: React.FC = () => {
  return (
    <NavigationContainer>
      <Stack.Navigator initialRouteName="GovBrAuth">
        <Stack.Screen 
          name="GovBrAuth" 
          component={GovBrAuth}
          options={{ title: 'Autenticação Gov.br' }}
        />
        <Stack.Screen 
          name="VotingInterface" 
          component={VotingInterface}
          options={{ title: 'Votação Eletrônica' }}
        />
        <Stack.Screen 
          name="VoteVerification" 
          component={VoteVerification}
          options={{ title: 'Verificação de Voto' }}
        />
      </Stack.Navigator>
    </NavigationContainer>
  );
};
```

### **Integração com Gov.br**
```typescript
// mobile/hooks/useGovBrAuth.ts
import { useMutation } from 'react-query';
import { OAuth2Client } from '@govbr/oauth2-client';

export const useGovBrAuth = () => {
  const oauthClient = new OAuth2Client({
    clientId: process.env.GOVBR_CLIENT_ID,
    redirectUri: process.env.GOVBR_REDIRECT_URI,
    scope: 'openid profile email cpf'
  });
  
  const authenticate = useMutation(
    async () => {
      // Fluxo OAuth2 com gov.br
      const authUrl = oauthClient.getAuthorizationUrl();
      
      // Abrir navegador para autenticação
      const result = await oauthClient.handleRedirect();
      
      return {
        accessToken: result.access_token,
        userInfo: result.user_info,
        cpf: result.cpf
      };
    }
  );
  
  const verifyIdentity = useMutation(
    async (cpf: string) => {
      // Verificar identidade no TSE
      const response = await fetch('/api/v1/tse/verify-identity', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ cpf })
      });
      
      if (!response.ok) throw new Error('Identity verification failed');
      return response.json();
    }
  );
  
  return {
    authenticate: authenticate.mutateAsync,
    verifyIdentity: verifyIdentity.mutateAsync,
    isAuthenticating: authenticate.isLoading,
    isVerifying: verifyIdentity.isLoading,
  };
};
```

### **Interface de Votação Mobile**
```typescript
// mobile/screens/VotingInterface.tsx
import React, { useState } from 'react';
import { View, Text, TouchableOpacity, ScrollView } from 'react-native';
import { CandidateCard } from '../components/CandidateCard';
import { VoteConfirmation } from '../components/VoteConfirmation';

export const VotingInterface: React.FC = () => {
  const [selectedCandidate, setSelectedCandidate] = useState(null);
  const [step, setStep] = useState<'select' | 'confirm' | 'receipt'>('select');
  
  const handleCandidateSelect = (candidate: Candidate) => {
    setSelectedCandidate(candidate);
    setStep('confirm');
  };
  
  const handleVoteConfirm = async () => {
    try {
      // Enviar voto para blockchain via API
      const response = await fetch('/api/v1/mobile/vote', {
        method: 'POST',
        headers: { 
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${getAuthToken()}`
        },
        body: JSON.stringify({
          candidateId: selectedCandidate.id,
          electionId: getCurrentElectionId(),
          deviceFingerprint: getDeviceFingerprint()
        })
      });
      
      if (!response.ok) throw new Error('Vote failed');
      
      const result = await response.json();
      setStep('receipt');
    } catch (error) {
      console.error('Vote error:', error);
    }
  };
  
  return (
    <View className="flex-1 bg-gray-50">
      {step === 'select' && (
        <ScrollView className="p-4">
          <Text className="text-2xl font-bold mb-6 text-center">
            Selecione seu candidato
          </Text>
          {candidates.map(candidate => (
            <CandidateCard
              key={candidate.id}
              candidate={candidate}
              onSelect={() => handleCandidateSelect(candidate)}
            />
          ))}
        </ScrollView>
      )}
      
      {step === 'confirm' && (
        <VoteConfirmation
          candidate={selectedCandidate}
          onConfirm={handleVoteConfirm}
          onBack={() => setStep('select')}
        />
      )}
      
      {step === 'receipt' && (
        <VoteReceipt
          candidate={selectedCandidate}
          transactionHash={voteResult.transactionHash}
        />
      )}
    </View>
  );
};
```

### **Segurança Mobile**
```typescript
// mobile/security/MobileSecurity.ts
import { Platform } from 'react-native';
import * as Keychain from 'react-native-keychain';
import * as Biometrics from 'react-native-biometrics';

export class MobileSecurity {
  static async storeCredentials(credentials: any) {
    // Armazenar credenciais no keychain
    await Keychain.setInternetCredentials(
      'fortis-mobile',
      credentials.username,
      credentials.password
    );
  }
  
  static async authenticateWithBiometrics() {
    // Autenticação biométrica
    const result = await Biometrics.authenticate({
      reason: 'Autentique-se para votar',
      fallbackLabel: 'Usar senha',
      disableDeviceFallback: false
    });
    
    return result.success;
  }
  
  static async generateDeviceFingerprint() {
    // Gerar fingerprint único do dispositivo
    const deviceInfo = {
      platform: Platform.OS,
      version: Platform.Version,
      model: await getDeviceModel(),
      uniqueId: await getUniqueId()
    };
    
    return hashDeviceInfo(deviceInfo);
  }
  
  static async encryptVoteData(voteData: any) {
    // Criptografar dados do voto
    const key = await this.getEncryptionKey();
    return encrypt(voteData, key);
  }
}
```

### **Roadmap Mobile**
- **Fase 1**: Autenticação gov.br e interface básica
- **Fase 2**: Integração com blockchain e segurança
- **Fase 3**: Testes e validação com TSE
- **Fase 4**: Deploy e monitoramento

---

## 🔗 **Referências Técnicas**

Para especificações técnicas completas e detalhes de implementação:

- **[FORTIS_ESPECIFICACAO_TECNICA.md](../../FORTIS_ESPECIFICACAO_TECNICA.md)** - Especificação técnica consolidada completa
- **[Urnas Transacionais](../urnas-transacionais/README.md)** - Integração com urnas eletrônicas

---

*Documentação Frontend FORTIS - Desenvolvida pelo Frontend Developer Agent*
