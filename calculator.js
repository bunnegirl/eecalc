const MEGA = 1000000;
const KILO = 1000;
const MILLI = 0.001;
const MICRO = 0.000001;
const NANO = 0.000000001;
const PICO = 0.000000000001;
const E6_TABLE = [
    1.0,
    1.5,
    2.2,
    3.3,
    4.7,
    6.8
];
const E12_TABLE = [
    1.0,
    1.2,
    1.5,
    1.8,
    2.2,
    2.7,
    3.3,
    3.9,
    4.7,
    5.6,
    6.8,
    8.2,
];
const E24_TABLE = [
    1.0,
    1.1,
    1.2,
    1.3,
    1.5,
    1.6,
    1.8,
    2.0,
    2.2,
    2.4,
    2.7,
    3.0,
    3.3,
    3.6,
    3.9,
    4.3,
    4.7,
    5.1,
    5.6,
    6.2,
    6.8,
    7.5,
    8.2,
    9.1
];
const E48_TABLE = [
    1.00,
    1.05,
    1.10,
    1.15,
    1.21,
    1.27,
    1.33,
    1.40,
    1.47,
    1.54,
    1.62,
    1.69,
    1.78,
    1.87,
    1.96,
    2.05,
    2.15,
    2.26,
    2.37,
    2.49,
    2.61,
    2.74,
    2.87,
    3.01,
    3.16,
    3.32,
    3.48,
    3.65,
    3.83,
    4.02,
    4.22,
    4.42,
    4.64,
    4.87,
    5.11,
    5.36,
    5.62,
    5.90,
    6.19,
    6.49,
    6.81,
    7.15,
    7.50,
    7.87,
    8.25,
    8.66,
    9.09,
    9.53
];
const NOT_SET = false;
const SET = true;

function capacitors_table(table, min, max) {
    let res = [];
    let mults = [
        // pico
        0.000000000001, 0.00000000001, 0.0000000001,
        // nano
        0.000000001, 0.00000001, 0.0000001,
        // micro
        0.000001, 0.00001, 0.0001,
        // milli
        0.001, 0.01, 0.1,
    ];

    for (let mult of mults) {
        for (let val of table) {
            val *= mult;

            if (val > min && val < max) {
                res.push(val);
            }
        }
    }

    return res;
}

function resistors_table(table, min, max) {
    let res = [];
    let mults = [0.1, 1, 10, 100, 1000, 10000, 100000, 1000000];

    for (let mult of mults) {
        for (let val of table) {
            val *= mult;

            if (val > min && val < max) {
                res.push(val);
            }
        }
    }

    return res;
}

function resistors_table(table, min, max) {
    let res = [];
    let mults = [0.1, 1, 10, 100, 1000, 10000, 100000, 1000000];

    for (let mult of mults) {
        for (let val of table) {
            val *= mult;

            if (val > min && val < max) {
                res.push(val);
            }
        }
    }

    return res;
}

function lookup(target_f_spec, target_q_spec, r1, target_r2, target_c1, target_c2, capacitors, resistors) {
    // f = 1 / (2 * pi * sqrt(l * c1))
    // q = 2 * pi * f * l / r1
    // l = r1 * r2 * c2

    let checked = [];
    let results = [];
    let target_l = 0.001;
    let target_f = target_f_spec[0];
    let min_f = target_f * (1 - target_f_spec[1]);
    let max_f = target_f * (1 + target_f_spec[1]);
    let target_q = target_q_spec[0];
    let min_q = target_q * (1 - target_q_spec[1]);
    let max_q = target_q * (1 + target_q_spec[1]);

    let r2_table = resistors;

    if (target_r2[0] == SET) {
        r2_table = [target_r2[1]];
    }

    let c1_table = capacitors;

    if (target_c1[0] == SET) {
        c1_table = [target_c1[1]];
    }

    let c2_table = capacitors;

    if (target_c2[0] == SET) {
        c2_table = [target_c2[1]];
    }

    while (target_l < 100) {
        for (let c1 of c1_table) {
            let f = 1 / (2 * Math.PI * Math.sqrt(target_l * c1));
            let q = 2 * Math.PI * f * target_l / r1;
            let min_l = target_l * 0.8;
            let max_l = target_l * 1.2;

            // exclude results too far off target
            if (q > min_q && q < max_q && f > min_f && f < max_f) {
                for (let r2 of r2_table) {
                    for (let c2 of c2_table) {
                        let key = c1 + "-" + c2 + "-" + r1 + "-" + r2;
                        let l = r1 * r2 * c2;

                        if (checked.includes(key)) continue;

                        let f = 1 / (2 * Math.PI * Math.sqrt(l * c1));
                        let q = 2 * Math.PI * f * l / r1;

                        if (l > min_l && l < max_l && q > min_q && q < max_q && f > min_f && f < max_f) {
                            let l_dist = target_l - l;
                            let f_dist = target_f - f;
                            let q_dist = target_q - q;

                            if (l_dist < 0) {
                                l_dist = 0 - l_dist;
                            }

                            if (f_dist < 0) {
                                f_dist = 0 - f_dist;
                            }

                            if (q_dist < 0) {
                                q_dist = 0 - q_dist;
                            }

                            let diff = ((target_l / l) + (target_f / f) + (target_q / q)) / 3;

                            if (diff < 0) {
                                diff = 0 - diff
                            }

                            results.push([
                                (f_dist + q_dist + l_dist) / 2,
                                l, f, q, c1, c2, r1, r2
                            ]);
                        }

                        checked.push(key);
                    }
                }
            }
        }

        target_l += 0.001;
    }

    results.sort((a, b) => {
        return a[0] - b[0];
    });

    return results.slice(0, 100);
}

function parse_capacitance(string) {
    let mult = 1;

    // picofarads
    if (string.includes("p")) {
        string = string.replace("p", ".");
        mult = PICO;
    }

    // nanofarad
    else if (string.includes("n")) {
        string = string.replace("n", ".");
        mult = NANO;
    }

    // microfarad
    else if (string.includes("u")) {
        string = string.replace("u", ".");
        mult = MICRO;
    }

    // millifarad
    else if (string.includes("m")) {
        string = string.replace("m", ".");
        mult = MILLI;
    }

    return parseFloat(string) * mult;
}

function parse_resistance(string) {
    let mult = 1;

    // kiloohms
    if (string.includes("k")) {
        string = string.replace("k", ".");
        mult = KILO;
    }

    // megaohms
    else if (string.includes("m")) {
        string = string.replace("m", ".");
        mult = MEGA;
    }

    return parseFloat(string) * mult;
}

function parse_frequency(string) {
    let mult = 1;

    // kiloherts
    if (string.includes("k")) {
        string = string.replace("k", ".");
        mult = KILO;
    }

    // megaherts
    else if (string.includes("m")) {
        string = string.replace("m", ".");
        mult = MEGA;
    }

    return parseFloat(string) * mult;
}

function format_frequency(frequency) {
    let mult = 1;
    let unit = "";

    if (frequency >= MEGA) {
        mult = MEGA;
        unit = "m";
    }

    else if (frequency >= KILO) {
        mult = KILO;
        unit = "k";
    }

    let value = (frequency / mult).toFixed(2).replace(/\.0+$/, "") + unit;

    return value;
}

function format_q(q) {
    return q.toFixed(2);
}

function format_capacitance(capacitance) {
    let mult = 1;
    let unit = "";

    if (capacitance >= 1) {
        // 
    }

    else if (capacitance >= MILLI) {
        mult = MILLI;
        unit = "m";
    }

    else if (capacitance >= MICRO) {
        mult = MICRO;
        unit = "u";
    }

    else if (capacitance >= NANO) {
        mult = NANO;
        unit = "n";
    }

    else if (capacitance >= PICO) {
        mult = PICO;
        unit = "p";
    }

    let value = (capacitance / mult).toFixed(1).replace(/(\.0+$|\.)/, unit);

    return value;
}

function format_resistance(resistance) {
    let mult = 1;
    let unit = "";

    if (resistance >= MEGA) {
        mult = MEGA;
        unit = "m";
    }

    else if (resistance >= KILO) {
        mult = KILO;
        unit = "k";
    }

    let value = (resistance / mult).toFixed(1).replace(/(\.0+$|\.)/, unit);

    return value;
}

function read_capacitance_series() {
    let input = document.getElementById("capacitance-series");

    switch (input.value) {
        case "e6": return E6_TABLE;
        case "e12": return E12_TABLE;
        case "e24": return E24_TABLE;
        case "e48": return E48_TABLE;
    }
}

function read_minimum_capacitance() {
    let input = document.getElementById("minimum-capacitance");
    let value = input.value == "" ? input.placeholder : input.value;

    return parse_capacitance(value);
}

function read_maximum_capacitance() {
    let input = document.getElementById("maximum-capacitance");
    let value = input.value == "" ? input.placeholder : input.value;

    return parse_capacitance(value);
}

function read_resistance_series() {
    let input = document.getElementById("resistance-series");

    switch (input.value) {
        case "e6": return E6_TABLE;
        case "e12": return E12_TABLE;
        case "e24": return E24_TABLE;
        case "e48": return E48_TABLE;
    }
}

function read_minimum_resistance() {
    let input = document.getElementById("minimum-resistance");
    let value = input.value == "" ? input.placeholder : input.value;

    return parse_resistance(value);
}

function read_maximum_resistance() {
    let input = document.getElementById("maximum-resistance");
    let value = input.value == "" ? input.placeholder : input.value;

    return parse_resistance(value);
}

function read_target_r1() {
    let input = document.getElementById("target-r1");
    let value = input.value == "" ? input.placeholder : input.value;

    return parse_resistance(value);
}

function read_target_r2() {
    let input = document.getElementById("target-r2");

    return input.value == "" ? [NOT_SET, null] : [SET, parse_resistance(input.value)];
}

function read_target_c1() {
    let input = document.getElementById("target-c1");

    return input.value == "" ? [NOT_SET, null] : [SET, parse_capacitance(input.value)];
}

function read_target_c2() {
    let input = document.getElementById("target-c2");

    return input.value == "" ? [NOT_SET, null] : [SET, parse_capacitance(input.value)];
}

function read_target_frequency() {
    let input = document.getElementById("target-frequency");
    let value = input.value == "" ? input.placeholder : input.value;
    let select = document.getElementById("target-frequency-tolerance");

    return [parse_frequency(value), parseFloat(select.value)];
}

function read_target_q() {
    let input = document.getElementById("target-q");
    let value = input.value == "" ? input.placeholder : input.value;
    let select = document.getElementById("target-q-tolerance");

    return [parseFloat(value), parseFloat(select.value)];
}

function clear_table() {
    let tbody = document.querySelector("table tbody");

    if (tbody) {
        tbody.parentNode.removeChild(tbody);
    }
}

function show_message() {
    let table = document.querySelector("table");
    let tbody = document.querySelector("table tbody");

    let new_tbody = document.createElement("tbody");
    let row = document.createElement("tr");
    let msg = document.createElement("td");

    msg.setAttribute("class", "msg");
    msg.setAttribute("colspan", "6");
    msg.textContent = "calculating combinations";

    row.appendChild(msg);
    new_tbody.appendChild(row);

    table.replaceChild(new_tbody, tbody);
}

function populate_table(results) {
    let table = document.querySelector("table");
    let tbody = document.querySelector("table tbody");
    let new_tbody = document.createElement("tbody");
    let rank = 1;

    for (let result of results) {
        let row = document.createElement("tr");

        let frequency_cell = document.createElement("td");

        frequency_cell.className = "frequency";
        frequency_cell.appendChild(document.createTextNode(format_frequency(result[2])));
        row.appendChild(frequency_cell);

        let q_cell = document.createElement("td");

        q_cell.className = "q";
        q_cell.appendChild(document.createTextNode(format_q(result[3])));
        row.appendChild(q_cell);

        let c1_cell = document.createElement("td");

        c1_cell.className = "value";
        c1_cell.appendChild(document.createTextNode(format_capacitance(result[4])));
        row.appendChild(c1_cell);

        let c2_cell = document.createElement("td");

        c2_cell.className = "value";
        c2_cell.appendChild(document.createTextNode(format_capacitance(result[5])));
        row.appendChild(c2_cell);

        let r1_cell = document.createElement("td");

        r1_cell.className = "value";
        r1_cell.appendChild(document.createTextNode(format_resistance(result[6])));
        row.appendChild(r1_cell);

        let r2_cell = document.createElement("td");

        r2_cell.className = "value";
        r2_cell.appendChild(document.createTextNode(format_resistance(result[7])));
        row.appendChild(r2_cell);

        new_tbody.appendChild(row);

        rank++;
    }

    if (results.length == 0) {
        let row = document.createElement("tr");

        let msg_cell = document.createElement("td");

        msg_cell.setAttribute("class", "msg");
        msg_cell.setAttribute("colspan", "6");
        msg_cell.textContent = "no possible combinations were found";

        row.appendChild(msg_cell);

        new_tbody.appendChild(row);
    }

    table.replaceChild(new_tbody, tbody);
}

window.onload = () => {
    let calculate = () => {
        show_message();

        setTimeout(() => {
            let capacitors = capacitors_table(
                read_capacitance_series(),
                read_minimum_capacitance(),
                read_maximum_capacitance()
            );
            let resistors = resistors_table(
                read_resistance_series(),
                read_minimum_resistance(),
                read_maximum_resistance()
            );

            let target_r1 = read_target_r1();
            let target_r2 = read_target_r2();
            let target_c1 = read_target_c1();
            let target_c2 = read_target_c1();
            let target_frequency = read_target_frequency();
            let target_q = read_target_q();

            let results = lookup(target_frequency, target_q, target_r1, target_r2, target_c1, target_c2, capacitors, resistors);

            populate_table(results);
        }, 1);
    };

    document.querySelector("button").onclick = calculate;
};