## Contexto de la aplicacion
Estoy desarrollando una aplicacion de software, principalmente en Laravel tipo RestfulAPI. En la cual
espero que puedan hacer consultas sobre el menu de productos y extras, obtener configuracion de como
se deben manejar los endpoint (algo como un discovery), y mediante un procesamiento externo,
un cliente pueda hacer pedidos de ese menu de opciones mediante un canal final, el cual incialmente
esta pensado para ser whatsapp. El diagrama funcional inicial, es que un cliente mande un whatsapp 
a nuestro numero de telefono, se le conteste el menu, se procese su pedido y se guarde en la API,
esta API notificara a otra aplicacion en el Restaurante (o la App de Restaurante consultara la API, 
esta aun por definir) para mostrar el pedido creado por el cliente y se pueda procesar.

## Planteamiento del problema de flujo
La idea original, es que la api solo se encargue del despliegue del menu, la toma de pedidos y posible
notificacion a listeners de los pedidos, o estos se consulten para ver si hay pendientes.
Aqui viene el problema, alguien tiene que recibir el mensaje de whatsapp y enviarlo a un agente, y el 
agente tomara las desciciones correspondientes y obtendra su comportameinto y configuracion de la API,
pero si este agente no ha consultado previamente la API, no tendra idea de que hacer.
La primera solucion propuesta fue, que mi API/Backend reciba mi mensaje y este backend instancie un 
agente de IA para la interpretacion de los mensajes, pero ahora se tendria que desarrollar la comunicacion
entre el backend y whatsapp, asi como la conexion entre el backend y los agentes de IA para la interpretacion
del lenguaje y toma de desciciones. Como la vez? Cual propuestas hay para solucionar este problema?

## Especificacion de desarollo actual
La API de laravel la quiero dejar como ya esta, quiero que solo me maneje informacion sobre el restaurante, sobre el menu, y guarde los pedidos y los notifique al POS del restaurante. Para whatsapp estoy utilizando EvolutionAPI, y este lo estoy enlazando a una aplicacion escrita en Rust para poder procesar todos los mensajes de whatsapp y poder generar algun historial y cantidad de creditos consumidos por cada cliente/restaurante.

## Posible solucion
Crear "tools" y revisar "actions" pedidas por un LLM para que pueda llamar a las herramientas de la API.


## Problemas de que un agente llame directamente a API:
Seguridad: El LLM necesita credenciales/API keys

Formato: El LLM puede enviar JSON mal formado

Validación: No hay validación intermedia

Logging: Difícil auditar qué está haciendo el LLM

Rate limiting: El LLM podría hacer spam a tus APIs