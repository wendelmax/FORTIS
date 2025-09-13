# FORTIS Backend - Dockerfile
FROM rust:1.75-slim as builder

# Instalar dependências do sistema
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Criar diretório de trabalho
WORKDIR /app

# Copiar arquivos de dependências
COPY Cargo.toml Cargo.lock ./

# Copiar código fonte
COPY src ./src
COPY migrations ./migrations

# Build da aplicação
RUN cargo build --release

# Imagem final
FROM debian:bookworm-slim

# Instalar dependências de runtime
RUN apt-get update && apt-get install -y \
    libssl3 \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Criar usuário não-root
RUN useradd -r -s /bin/false fortis

# Criar diretório de trabalho
WORKDIR /app

# Copiar binário compilado
COPY --from=builder /app/target/release/fortis-backend /app/fortis-backend

# Copiar migrações
COPY --from=builder /app/migrations /app/migrations

# Alterar propriedade dos arquivos
RUN chown -R fortis:fortis /app

# Mudar para usuário não-root
USER fortis

# Expor porta
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Comando de inicialização
CMD ["./fortis-backend"]
