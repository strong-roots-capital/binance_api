# Binance API
OpenAPI 3.0 (Swagger) schemas for Binance

A swagger schemas for Binance's API as per there [docs](https://github.com/binance-exchange/binance-official-api-docs). [Swagger toolbox](https://swagger-toolbox.firebaseapp.com/) was used to convert json to OpenAPI compatible YAML.

Binance (Amazon CloudFront) make use of CORS, in order to use the [online swagger editor](https://editor.swagger.io), chrome have to be started with the `disable-web-security` flag:<br>
```chrome.exe --user-data-dir="C:/Chrome dev session" --disable-web-security```

[OpenAPI Generator](https://github.com/OpenAPITools/openapi-generator) can be used to generate a client of server for OpenAPI 3.0. It is a template-driven engine to generate documentation, API clients and server stubs in different languages by parsing your OpenAPI definition (community-driven fork of swagger-codegen). Example of the code generation: `java -jar openapi-generator-cli-4.0.0.jar generate -i binance.yml -o test/ -g rust`
