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