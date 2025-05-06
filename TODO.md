# TODO

* Should we pass the taxonomy & class name to the redactor when redacting an object?
* Should we support serializing a Classified<T>?
* Right now, RedactionEngine.redact() does three dynamic dispatches to the
  supplied Classified instance. Should try to reduce that to 2 or maybe even 1.
