#define_import_path heart::utils

fn pos_animate(pos:vec3<f32>, t: f32) -> vec3<f32> {
    var speed = 5.0 * t;

    // normalize to min - max
    var max = 1.1;
    var min = 0.9;
    var m = sin(speed + sin(speed) + sin(speed)) * 0.5 + 0.5;
    var nor_m = m * (max - min) + min;

    return pos * nor_m;
}
