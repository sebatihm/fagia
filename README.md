# Instrucciones para el funcionamiento
- Instala sea orm
- - cargo install sea-orm-cli@1.1.0
- Levanta un servicio de SQL
- Crea un BD de nombre "fagia"
- cargo run

## Funcionamiento
Se utilizaron la libreria de sea-orm para manejar la conexion a la base de datos, se conecta a una base de datos SQL llamada "fagia" y la de jsonwebtoken para manejar la autenticacion de usuarios.
Una vez se tenga la instancia de la base de datos se iniciara.

![image](https://github.com/user-attachments/assets/2c77ef35-89e6-460d-a204-d124c32dd24b)


## Rutas (Endpoints)
![image](https://github.com/user-attachments/assets/496df8e1-9199-48a6-9b5a-1b3667bfe7e2)

### Sin proteccion de rutas
- /login                    [POST]    Logueo
- /register-donator         [POST]    Registro de donadores
- /register-beneficiary     [POST]    Registro de beneficiarios

### Proteccion de rutas autenticadas
- /donation                [GET]      Ver las donaciones donde participa el usuario (index)

#### Proteccion de rol (Donator)
- /aliments                [GET]      Ver los alimentos del donador
- /aliments                [POST]     Registrar alimentos del donador
- /aliments/{ID}           [POST]     Mostrar un alimento con {ID} del donador
- /aliments/{ID}           [DELETE]   Mostrar un alimento con {ID} del donador
- /donation                [POST]     Crear donaciones

#### Proteccion de rol (Beneficiary)
- /beneficiary/donation/filter/{DAYS} [GET]       Filtrar las donaciones por un numero {DAYS} de dias
- /beneficiary/donation/{ID}/donator  [GET]       Obtener la informacion de un donador de una donacion en especifico
