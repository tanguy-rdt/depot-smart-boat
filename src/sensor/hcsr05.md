# Capteur de profondeur

![](./img/main.png)


Encore une fois nous nous retrouvons sur l'écran principal. \
Cette fois-ci pour une simulation d'un sonar réalisé à l'aide d'un capteur ultrason, le _HC-SR05_.

## Fonctionnement du HC-SR05

Ici, aucune configuration n’est nécessaire. Nous allons émettre une onde que nous pouvons métaphoriquement imaginer à l'aide d'un 1 booléen. Pour déterminer la distance par rapport aux profondeurs, nous allons mesurer le temps mis par l'onde pour revenir vers le récepteur. \
Seuls les gpio sont utilisés, on émet une impulsion et on la reçois, la distance est ensuite calculée en fonction de la différence de temps et la vitesse du son. \
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
