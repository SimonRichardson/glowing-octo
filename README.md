# glowing-octo


### Get `/events`

```sh
curl http://127.0.0.1:8080/events
```

### Post `/events`

```sh
curl -X POST -H "Accept: application/json" -H "Content-Type: application/json" -d "{\"name\":\"**Awesome**\"}" http://127.0.0.1:8080/events
```

### Delete `/events`

```sh
curl -X DELETE http://127.0.0.1:8080/events
```