[ENGLISH]

----
----

[SPANISH]

En la parte izquierda estamos almacenando el </b>s1</b>
En primer lugar, una parte en </b>stack</b>, que está almecanando en el </b>s1</b>, el <b>puntero (ptr)</b> que apunta, hacia el valor que está en el heap, y también estoy guardando el <b>largo (len)</b> y su <b>capacidad (capacity)</b>. Estas 3 caracteristicas siempre se van a guardar dentro de una variable se almacenan en el <b>heap</b>, est tipo de variables que no se sabe el largo.
Y algo interesante, el puntero (ptr) apunta a la sección del <b>heap</b> y al valor final.

Si vemos la parte derecha, es el valor de la variable <b>s1</b> que está en el <b>heap</b>

<h3>s1</h3>

<table>
<tr>
<td>

|    name   | value |
|-----------|-------|
|   ptr     |       |
|   len     |   5   |
| capacity  |   5   |

</td>
<td style="padding: 0 20px; font-size: 24px;">
==>
</td>
<td>

| index | value |
|-------|-------|
|   0   |   h   |
|   1   |   e   |
|   2   |   l   |
|   3   |   l   |
|   4   |   o   |

</td>
</tr>
</table>

Ahora ¿Que ocurre cuando almacenamos y hacemos referencia en <b>s2</b> y <b>s1</b>?

<table>
<tr>
<td>
<h3>s1</h3>
|    name   | value |
|-----------|-------|
|   ptr     |       |
|   len     |   5   |
| capacity  |   5   |

</td>
<td>
<h3>s2</h3>
|    name   | value |
|-----------|-------|
|   ptr     |       |
|   len     |   5   |
| capacity  |   5   |

</td>
<td style="padding: 0 20px; font-size: 24px;">
==>
</td>
<td>

| index | value |
|-------|-------|
|   0   |   h   |
|   1   |   e   |
|   2   |   l   |
|   3   |   l   |
|   4   |   o   |

</td>
</tr>
</table>

Lo que ocurre es que <b>s2</b> se crea en el <b>stack</b> con estas 3 propiedades y el puntero, apunta a la misma espacio de memoria, que tiene <b>s1</b>, entonces ambas variables que tienen almacenado estas 3 propiedades, el valor apunta a la misma dirección de memoria en el <b>heap</b>, asi funciona en <b>Rust</b>, este tipo de variables que se almacenan en el <b>heap</b>. Uno podria pensar, de que cada una de las variables va a tener su propio <b>stack</b> y su propio valor en el <b>heap</b>.

Esto en cierta forma es muy costoso, cuando estamos tratando estas variables que no tienen un largo definido.