# envoy-exporter
[Prometheus](https://prometheus.io) exporter for [Enphase Envoy Gateway](https://enphase.com/en-us/products-and-services/envoy).

This exporter is not endorsed by or approved by Enphase.

Features:

* Can poll multiple Envoy gateways in a single metrics call
* Exports Envoy data such as current watts, today's watt hours and lifetime watt hours
* Exports individual inverter data such as each serial number and last-reported watts

## Usage

```
envoy-exporter [config_file]
```

The format of `config_file` is shown in the `etc` directory.

The password for the Envoy is typically the last six characters of the serial
number. The serial number is available from the Envoy's public web interface.
