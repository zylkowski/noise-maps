## noise-maps
Library that let's you create compound noise maps by defining your own noise generators and operations(look in the tests, they may serve as simple example).
Example images:

![NoiseMap1](/images/noise_1.png)
![NoiseMap2](/images/noise_2.png)

Best thing about this crate is that all noise operators and noise configs have to be serializable. Because of this you can store noise maps "recipes" and use them as you wish.
(This crate probably doesn't support wasm because of `typetag` dependency)
