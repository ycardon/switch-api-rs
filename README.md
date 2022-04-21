# switch-api-rs

A simple REST API reporting MacBook CPU, power and display status
- rewrite in Rust of my Node.js project [switch-api](https://github.com/ycardon/switch-api)
- adapted to work on MacBook ARM64 architecture
- used in conjunction with [Home Assistant](https://www.home-assistant.io) [RESTful Switch](https://home-assistant.io/components/switch.rest) integration

## usage and integration inside home-assistant

### control mac display

- `GET /display` : get current screen state, sleeping or not (tested on MacBook ARM64 architecture)
- `POST /display body=ON|OFF` : switch screeen state

```yaml
switch:
  - platform: rest
    name: Macbook Display
    resource: 'http://mymac:8182/display'
    scan_interval: 30
```

### get battery power status (on macOS Mojave)

- `GET /power`

```yaml
sensor:
  - platform: rest
    name: Macbook battery
    resource: 'http://mymac:8182/power'
    json_attributes:
      - isOnAC
      - isOnBattery
      - isCharged
      - chargingStatus
      - chargePercent
      - remainingChargeTime
    value_template: '{{ value_json.chargePercent }}'
    unit_of_measurement: '%'
    scan_interval: 120
```

### get cpu average on 1mn

- `GET /cpu`

```yaml
sensor:
  - platform: rest
    name: Macbook CPU
    resource: 'http://mymac:8182/cpu'
    value_template: '{{ value | round(1) }}'
    unit_of_measurement: '%'
    scan_interval: 60
```

## installation

```shell
# build from source
git clone https://github.com/ycardon/switch-api-rs
cd switch-api-rs
cargo install --path .

# start with user session
<edit to adjust path> local.switch-api-rs.plist
cp local.switch-api-rs.plist ~/Library/LaunchAgents/
launchctl load Library/LaunchAgents/local.switch-api-rs.plist
launchctl start local.switch-api-rs
```

## useful commands

- sleep display: `pmset displaysleepnow`
- wake display: `caffeinate -u -t 1`
- test display state : `pmset -g powerstate | grep DCPDPDeviceProxy | wc -l` result <1 is sleeping
- battery power status : `pmset -g batt` (and some parsing)

TODO scan_interval: 10