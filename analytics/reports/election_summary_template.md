# RELATÓRIO DE ELEIÇÃO - {election_name}

**Data de Geração:** {generation_date}  
**Sistema:** FORTIS - Votação Eletrônica Brasileira  
**Versão:** 1.0

---

## 📊 RESUMO EXECUTIVO

| Métrica | Valor |
|---------|-------|
| **Total de Votos** | {total_votes:,} |
| **Eleitores Únicos** | {unique_voters:,} |
| **Taxa de Participação** | {participation_rate}% |
| **Status** | {election_status} |
| **Score de Segurança** | {security_score}/100 |

---

## 🗳️ RESULTADOS POR CANDIDATO

| Candidato | Partido | Votos | Percentual |
|-----------|---------|-------|------------|
{results_table}

---

## ⏰ ANÁLISE TEMPORAL

### Pico de Votação
- **Horário de Maior Movimento:** {peak_hour}
- **Votos no Pico:** {peak_votes:,}
- **Média por Hora:** {avg_hourly_votes:,}

### Distribuição por Hora
```
{hourly_distribution}
```

---

## 🗺️ ANÁLISE GEOGRÁFICA

### Estados com Maior Participação
1. **{top_state_1}:** {top_rate_1}% ({top_votes_1:,} votos)
2. **{top_state_2}:** {top_rate_2}% ({top_votes_2:,} votos)
3. **{top_state_3}:** {top_rate_3}% ({top_votes_3:,} votos)

### Média Nacional
- **Taxa Média de Participação:** {avg_participation}%
- **Desvio Padrão:** {participation_std}%

---

## 🔒 MÉTRICAS DE SEGURANÇA

| Métrica | Valor | Status |
|---------|-------|--------|
| **Votos Verificados** | {verified_votes:,} ({verification_rate}%) | {verification_status} |
| **Votos Auditados** | {audited_votes:,} ({audit_rate}%) | {audit_status} |
| **Votos Suspeitos** | {suspicious_votes} | {suspicious_status} |
| **Anomalias Detectadas** | {anomalies_detected} | {anomaly_status} |

### Análise de Anomalias
{anomaly_analysis}

---

## 📈 PADRÕES IDENTIFICADOS

### Tendências de Votação
{ voting_patterns }

### Comportamento dos Eleitores
{ voter_behavior }

### Análise de Candidatos
{ candidate_analysis }

---

## 🎯 RECOMENDAÇÕES

### Segurança
{ security_recommendations }

### Processo Eleitoral
{ process_recommendations }

### Melhorias Futuras
{ improvement_recommendations }

---

## 📋 ANEXOS

### Gráficos Gerados
- [Gráfico de Candidatos](candidates_chart.png)
- [Timeline de Votação](timeline_chart.png)
- [Análise Geográfica](geographic_chart.png)
- [Métricas de Segurança](security_chart.png)

### Dados Brutos
- [Dados de Votação](votes_data.csv)
- [Dados Geográficos](geographic_data.csv)
- [Logs de Segurança](security_logs.csv)

---

## 🔍 METODOLOGIA

### Coleta de Dados
- **Fonte:** Sistema FORTIS
- **Período:** {data_period}
- **Validação:** {validation_method}

### Análise Estatística
- **Modelo de Participação:** Random Forest
- **Detecção de Anomalias:** Isolation Forest
- **Clustering:** DBSCAN
- **Confiança:** 95%

### Auditoria
- **Verificação:** {audit_verification}
- **Assinatura Digital:** {digital_signature}
- **Hash do Relatório:** {report_hash}

---

*Relatório gerado automaticamente pelo Sistema FORTIS*  
*Para dúvidas ou esclarecimentos, entre em contato com a equipe técnica*
