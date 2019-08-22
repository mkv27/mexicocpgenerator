# Mexico Postal Code Generator

## Usage
Set limit `0` to generate all postal codes, default is 4
```
./mexicocpgenerator <limit>
```

### Dev
```sh
cargo run
```

### Example cps.txt
```
line 1:
d_codigo|d_asenta|d_tipo_asenta|D_mnpio|d_estado|d_ciudad|d_CP|c_estado|c_oficina|c_CP|c_tipo_asenta|c_mnpio|id_asenta_cpcons|d_zona|c_cve_ciudad

line 2:
01000|San Ángel|Colonia|Álvaro Obregón|Ciudad de México|Ciudad de México|01001|09|01001||09|010|0001|Urbano|01
```