actix-proxy
===========

A rust libary for the ``actix-web`` framework. Sometimes you want to
implement a gateway or proxy which makes a request to some remote
service and simply forwards it to the client that made the request to
the proxy (in case you do not wish to or cannot return a redirection
to the remote location).
``actix-web`` provides the ``actix_web::client::Client``
for making Http requests from your runtime. Unfortunately,
``actix_web::client::ClientResponse`` does not implement the
``Responder`` trait, making it hard to simply forward the response
from the remote location, through the proxy to the client.
This library provide the ``IntoHttpResponse`` trait which transforms
a ``ClientResponse`` into a ``HttpResponse``.
With this trait, all you need to do is call ``.into_http_response()``
on your ``ClientResponse`` to forward the response from the remote
service back to the client.

TODO
----

* clippy setup

* documentation

* pipeline

* codecov

* publish
