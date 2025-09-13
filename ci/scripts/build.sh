#!/bin/bash

# FORTIS Build Script
# Builds all components of the FORTIS system

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Configuration
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BUILD_DIR="${PROJECT_ROOT}/build"
DOCKER_REGISTRY="${DOCKER_REGISTRY:-fortis}"
VERSION="${VERSION:-latest}"

# Create build directory
mkdir -p "${BUILD_DIR}"

log_info "Starting FORTIS build process..."
log_info "Project root: ${PROJECT_ROOT}"
log_info "Build directory: ${BUILD_DIR}"
log_info "Docker registry: ${DOCKER_REGISTRY}"
log_info "Version: ${VERSION}"

# Function to build backend
build_backend() {
    log_info "Building FORTIS Backend..."
    
    cd "${PROJECT_ROOT}/backend"
    
    # Build Rust backend
    log_info "Compiling Rust backend..."
    cargo build --release
    
    # Build Docker image
    log_info "Building Docker image for backend..."
    docker build -f ci/docker/Dockerfile.backend -t "${DOCKER_REGISTRY}/backend:${VERSION}" .
    
    log_success "Backend build completed"
}

# Function to build frontend
build_frontend() {
    log_info "Building FORTIS Frontend..."
    
    cd "${PROJECT_ROOT}/frontend"
    
    # Install dependencies
    log_info "Installing frontend dependencies..."
    npm ci
    
    # Build React app
    log_info "Building React application..."
    npm run build
    
    # Build Docker image
    log_info "Building Docker image for frontend..."
    docker build -f ci/docker/Dockerfile.frontend -t "${DOCKER_REGISTRY}/frontend:${VERSION}" .
    
    log_success "Frontend build completed"
}

# Function to build mobile
build_mobile() {
    log_info "Building FORTIS Mobile..."
    
    cd "${PROJECT_ROOT}/mobile"
    
    # Install dependencies
    log_info "Installing mobile dependencies..."
    npm ci
    
    # Build for Android
    log_info "Building Android app..."
    npx react-native run-android --mode=release
    
    # Build for iOS (if on macOS)
    if [[ "$OSTYPE" == "darwin"* ]]; then
        log_info "Building iOS app..."
        npx react-native run-ios --mode=release
    else
        log_warning "Skipping iOS build (not on macOS)"
    fi
    
    log_success "Mobile build completed"
}

# Function to build blockchain
build_blockchain() {
    log_info "Building FORTIS Blockchain..."
    
    cd "${PROJECT_ROOT}/blockchain"
    
    # Install dependencies
    log_info "Installing blockchain dependencies..."
    npm ci
    
    # Compile contracts
    log_info "Compiling smart contracts..."
    npm run compile
    
    # Run tests
    log_info "Running blockchain tests..."
    npm test
    
    log_success "Blockchain build completed"
}

# Function to build AI components
build_ai() {
    log_info "Building FORTIS AI..."
    
    cd "${PROJECT_ROOT}/ai"
    
    # Install Python dependencies
    log_info "Installing AI dependencies..."
    pip install -r requirements.txt
    
    # Run tests
    log_info "Running AI tests..."
    python -m pytest tests/
    
    log_success "AI build completed"
}

# Function to build analytics
build_analytics() {
    log_info "Building FORTIS Analytics..."
    
    cd "${PROJECT_ROOT}/analytics"
    
    # Install Python dependencies
    log_info "Installing analytics dependencies..."
    pip install -r requirements.txt
    
    # Run tests
    log_info "Running analytics tests..."
    python -m pytest tests/
    
    log_success "Analytics build completed"
}

# Function to create deployment package
create_deployment_package() {
    log_info "Creating deployment package..."
    
    # Create deployment directory
    DEPLOY_DIR="${BUILD_DIR}/deployment"
    mkdir -p "${DEPLOY_DIR}"
    
    # Copy Kubernetes manifests
    cp -r "${PROJECT_ROOT}/ci/kubernetes" "${DEPLOY_DIR}/"
    
    # Copy Docker Compose files
    cp "${PROJECT_ROOT}/ci/docker/docker-compose.yml" "${DEPLOY_DIR}/"
    
    # Copy configuration files
    cp -r "${PROJECT_ROOT}/config" "${DEPLOY_DIR}/"
    
    # Create deployment script
    cat > "${DEPLOY_DIR}/deploy.sh" << 'EOF'
#!/bin/bash
set -e

echo "Deploying FORTIS system..."

# Deploy to Kubernetes
kubectl apply -f kubernetes/

# Or deploy with Docker Compose
# docker-compose up -d

echo "Deployment completed!"
EOF
    
    chmod +x "${DEPLOY_DIR}/deploy.sh"
    
    log_success "Deployment package created at ${DEPLOY_DIR}"
}

# Main build process
main() {
    log_info "Starting FORTIS build process..."
    
    # Build all components
    build_backend
    build_frontend
    build_mobile
    build_blockchain
    build_ai
    build_analytics
    
    # Create deployment package
    create_deployment_package
    
    log_success "FORTIS build process completed successfully!"
    log_info "All components built and ready for deployment"
}

# Run main function
main "$@"
