# Jenkins

Esta rama esta dedicada a la funcionalidades de jenkins, simplemente en esta esta el Jenkinsfile el cual realiza el git clone a esta rama, las pruebas y en dado caso que se descomente la 
etapa (ests simula el despliegue como se vio en clase).

```
pipeline {
    agent any

    environment {
        ADDRESS = 'localhost'
        PORT = '8081'
        DATABASE_URL = 'mysql://root:root@db:3306/fagia'
        SECRET = 'mysecretkey'
    }

    stages {
        stage('Checkout') {
            steps {
                script {
                    git branch: 'jenkins', url: 'https://github.com/sebatihm/fagia.git'
                }
            }
        }

        stage('Build') {
            steps {
                script {
                    // Construir el proyecto con cargo
                    
                    
                    
                    
                    sh 'cargo build --release'
                    
                    sh 'echo ::::::::::::::::::Migraciones::::::::::::::::::::::::::'
                    sh 'cargo install sea-orm-cli@1.1.0'
                    sh 'cargo run -p migration fresh'
                }
            }
        }

        stage('Test') {
            steps {
                script {
                    // Ejecutar las pruebas con cargo
                    sh 'cargo test'
                }
            }
        }

        // stage('Run') {
        //     steps {
        //         script {
        //             // Ejecutar el proyecto con cargo
        //             sh 'cargo run'
        //         }
        //     }
        // }
    }
}
```


## Docker y jenkins
La verdad investigue un cliente de docker el cual pudiera levantar un servicio de jenkins y como no encontre alguna imagen con Rust instalado, lo que hice fue crear una imagen de
jenkins con rust, en este dockerfile usando de platilla la imagen mas reciente de jenkins, instalo las dependencias que necesita rust para funcionar y despues al cambiarme al usuario
jenkins para que me deje usar cargo (compilador de rust) instalo rust y despues compruebo que funciona la instalación de rust.
```
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
```



El docker compose se encarga de levantar un servicio de jenkins y una base de datos relacional, que se necesita para que funcione la api rest.
```
services:
  db:
    image: mysql:8
    container_name: db
    restart: always
    environment:
      MYSQL_ROOT_PASSWORD: root
      MYSQL_DATABASE: fagia
    ports:
      - "3306:3306"
    networks:
      - rustnet

  jenkins:
    image: sebatihm/jenkins-with-rust:latest
    container_name: rust-kins
    depends_on:
      - db
    ports:
      - "8080:8080"
      - "50000:50000"
    networks:
      - rustnet

networks:
  rustnet:
```
