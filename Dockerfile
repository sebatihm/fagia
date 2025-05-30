FROM jenkins/jenkins:lts

USER root

# Dependencias necesarias para compilar
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    libpq-dev \
    libmariadb-dev \
    ca-certificates \
    git \
    && apt-get clean

# Cambiar a usuario jenkins
USER jenkins
ENV USER=jenkins

# Instalar Rust como usuario jenkins
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Añadir Cargo y Rust al PATH del usuario jenkins
ENV PATH="/var/jenkins_home/.cargo/bin:$PATH"

# Verificar instalación
RUN rustc --version && cargo --version
