# Capteur de profondeur

![](./img/main.png)


Encore une fois nous nous retrouvons sur l'écran principale. \
Cette fois-ci pour une simulation d'un sonar réalisé a l'aide d'un capteur ultrason, le _HC-SR05_.

## Fonctionnement du HC-SR05

Ici, aucune configuration est nécessaire. Nous allons émettre une onde que nous pouvons métaphoriquement imaginé à l'aide d'un 1 boolèen. Pour déterminer la distance par rapport au profondeur, nous allons mesurer le temps mis par l'onde pour revenir vers le récepteur. \
Seul les gpio sont utilisés, on emet une impulsion et on la recois, la distance est ensuite calculé en fonction de la différence de temps et la vitesse du son. \
L'implémentation en rust donne le code suivant.

```rust 
fn read_value(&self, gpio: &mut Gpio) -> f64 {
    gpio.set_high(self.trigger_pin);
    thread::sleep(Duration::from_micros(10));
    gpio.set_low(self.trigger_pin);

    while gpio.is_low(self.echo_pin) {}
    let start = std::time::Instant::now();

    while gpio.is_high(self.echo_pin) {}
    let duration = start.elapsed();

    duration.as_secs_f64() * 340.0 / 2.0 * 100.0
}
```