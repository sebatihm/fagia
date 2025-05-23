# Fagia BackEnd

Fagia es una iniciativa que busca apoyar a instituciones benéficas a contactar con
donadores de alimentos de una manera rápida y sencilla, utilizando un API Rest
para manejar las cantidades de alimentos donados, la información de las
organizaciones y la de los donadores.

Se busca evitar el desperdicio de comida por parte de personas de la comunidad
(principalmente restaurantes o supermercados que desechan alimentos todos los
días) mediante la API de Fagia, permitiendo redireccionar los recursos para que
sean aprovechados por instituciones caritativas.

## Requerimientos de instalación
- Rust version mas reciente [2025] (1.27.1 rustup)
- Instala sea orm mediante cargo
  `cargo install sea-orm-cli@1.1.0`
- Levanta un servicio de SQL
- Crea un BD de nombre "fagia"
- Añade el archivo .env con las siguientes variables de entorno
  - ADDRESS: Dirrecion ip donde se ejecutara el servicio (se recomienda 127.0.0.1 ó localhost)
  - PORT: Puerto en el que se ejecutara el servicio http
  - SECRET : Cualquier cadena
  - DATABASE_URL : mysql://user:password@host:puerto/nombre_bd
- Ejecuta el codigo `cargo run`

## Funcionamiento
Se utilizaron la libreria de sea-orm para manejar la conexion a la base de datos, se conecta a una base de datos SQL llamada "fagia" esto mediante un orm para manejar los modelos de manera efectiva y la libreria jsonwebtoken para manejar la autenticacion de usuarios mediante jwt.
Una vez se tenga la instancia de la base de datos se iniciara el servicio http.

![image](https://yt3.ggpht.com/SQAFFsDQULtOKNB4Qs1zDmpsoe5EYFT_YTrB-Ks3gLo6fSdhSWy7X6WsB2wuafZOc2F1E7Eeu8XDdA=s498-nd-v1)


## Rutas (Endpoints)
![image](https://github.com/user-attachments/assets/496df8e1-9199-48a6-9b5a-1b3667bfe7e2)

### Sin proteccion de rutas
- /login                    [POST]    Logueo
- /register-donator         [POST]    Registro de donadores
- /register-beneficiary     [POST]    Registro de beneficiarios

### Proteccion de rutas autenticadas
- /donation                [GET]      Ver las donaciones donde participa el usuario (index)
- /beneficiaries           [GET]      Ver los beneficiarios con cuenta activa (con credenciales)

#### Proteccion de rol (Donator)
- /aliments                [GET]      Ver los alimentos del donador
- /aliments                [POST]     Registrar alimentos del donador
- /aliments/{ID}           [POST]     Mostrar un alimento con {ID} del donador
- /aliments/{ID}           [DELETE]   Mostrar un alimento con {ID} del donador
- /donation                [POST]     Crear donaciones

#### Proteccion de rol (Beneficiary)
- /beneficiary/donation/filter/{DAYS} [GET]       Filtrar las donaciones por un numero {DAYS} de dias
- /beneficiary/donation/{ID}/donator  [GET]       Obtener la informacion de un donador de una donacion en especifico


## Dockerizacion
Para saber mas acerca de la dockerización de este proyecto se recomienda cambiar a la rama docker para su debida explicación. 

![image](https://github.com/user-attachments/assets/f040e253-3f6f-4f63-8aa8-9348fd700f75)



[Rama Docker](https://github.com/sebatihm/fagia/tree/docker)
