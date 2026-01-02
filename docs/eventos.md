

# Eventos

### Eventos de chats

- chats.update
- chats.upsert
- chats.delete

### Eventos de mensajes

- messages.upsert
- messages.update
- messages.delete

### Eventos de contactos

- contacts.upsert
- contacts.update
- contacts.delete

# Flujo de Eventos

## Al mandar un mensaje
- **recibir o generar el mensaje** : No es un evento como tal de evolution o whatsapp, es cuando nosotros tenemos un mensaje para enviar
- **send.message** : Indicando que el mensaje se envio
- **messages.update** : Indicando una actualizacion en el estado del mensaje, normalmente, que se entrego ( status: "DELIVERY_ACK") o que se leyo (status: "READ")
- **chats.upsert** : Indicando que se inserto el mensaje en el chat
- **messages.update** : Indicando una actualizacion en el estado del mensaje, normalmente, que se leyo (status: "READ")

## Al recibir un mensaje

## Eventos aleatorios
- **connection.update** : Al cambiar el stado de la conexion, posibles valores: state: "connecting", "open", ... ??
- **contacts.update** : Al cambiar foto de perfil de un contacto, posibles valores: "remoteJid" = "alternativeJid" // @lid, "profilePicUrl" = "https://pps.whatsapp.net/v/t61.24694-24/..."
- **messages.update** : Al cambiar el estado de un mensaje, posibles valores: "status" : "DELIVERY_ACK", "READ"