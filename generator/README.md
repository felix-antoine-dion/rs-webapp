Pour obtenir un template initial pour modification par la suite:
```bash
docker run --rm \ # stoppe le container après utilisation
  -v ${PWD}:/local \ # mappe un volume sur le répertoire courant
  --user "$(id -u):$(id -g)" \ # demande à docker de rouler avec l'utilisateur courant pour éviter des problèmes de permissions par la suite
  openapitools/openapi-generator-cli:v5.0.0 \ # identifie la version de l'image à exécuter
  author template \ # demande de générer le gabarit pour le langage suivant:
  -g rust-server \
  -o /local/template \ # le résultat sera envoyé dans un répertoire out (qui apparaitra sous le répertoire courant)
```
Pour personnaliser le gabarit, se référer à l'[article sur Github](https://github.com/OpenAPITools/openapi-generator/blob/master/docs/customization.md).

Pour lancer une génération de code à partir d'un fichier yaml et le gabarit:
```bash
docker run --rm \ # stoppe le container après utilisation
  -v ${PWD}:/local \ # mappe un volume sur le répertoire courant
  --user "$(id -u):$(id -g)" \ # demande à docker de rouler avec l'utilisateur courant pour éviter des problèmes de permissions par la suite
  openapitools/openapi-generator-cli:v5.0.0 \ # identifie la version de l'image à exécuter
  generate \ # indique que l'on veut générer le code pour:
  -g rust-server \
  -i https://raw.githubusercontent.com/OAI/OpenAPI-Specification/master/examples/v2.0/json/petstore-minimal.json \
  -o /local/out \ # le résultat sera envoyé dans un répertoire out (qui apparaitra sous le répertoire courant)
  -t /local/template \ # le chemin où le gabarit personnalisé se trouve
```