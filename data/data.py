import json
import os
import re
from astroquery.jplhorizons import Horizons
from astropy import units as u


# Using ID's as using names directly leads to ambiguity in some cases.
bodies = (
    (
        "largebody",  # planets
        [
            ("199", []),  # Mercury
            ("299", []),  # Venus
            ("399", ["Luna"]),  # Earth
            ("499", ["Phobos", "Deimos"]),  # Mars
            (
                "599",  # Jupiter
                [
                    "Io",
                    "Europa",
                    "Ganymede",
                    "Callisto",
                    "Metis",
                    "Amalthea",
                    "Andrastea",
                    "Thebe",
                ],
            ),
            ("699", []),
            ("799", []),
            ("899", []),
        ],
    ),
    (
        "smallbody",  # dwarf planets, asteroids
        [
            ("999", []),  # Pluto
            ("136199", []),  # Eris
            ("136472", []),  # Makemake
            ("136108", []),  # Haumea
            ("2000001", []),  # Ceres
        ],
    ),
)

color_map = {
    "Mercury": [0.5, 0.5, 0.5, 1.0],
    "Venus": [0.9, 0.8, 0.6, 1.0],
    "Earth": [0.0, 0.5, 1.0, 1.0],
    "Mars": [0.8, 0.3, 0.2, 1.0],
    "Jupiter": [0.9, 0.6, 0.4, 1.0],
    "Saturn": [0.8, 0.7, 0.5, 1.0],
    "Uranus": [0.6, 0.9, 0.8, 1.0],
    "Neptune": [0.2, 0.3, 0.9, 1.0],
    "Pluto": [0.7, 0.5, 0.3, 1.0],
    "Eris": [0.8, 0.8, 0.8, 1.0],
    "Haumea": [1.0, 1.0, 1.0, 1.0],
    "Makemake": [0.6, 0.3, 0.2, 1.0],
    "Ceres": [0.5, 0.4, 0.3, 1.0],
}


def get_mass(line: str) -> float | None:
    """Get the mass of the body in solar masses."""

    match = re.search(
        r"Mass\s*x\s*10\^(\d+)\s*\(\s*(g|kg)\s*\)\s*=\s*([\d\.]+)", line, re.IGNORECASE
    )

    if match:
        exponent = int(match.group(1))
        unit = match.group(2)
        value = float(match.group(3))

        if unit == "g":  # convert to kg
            exponent -= 3

        mass_kg = value * (10**exponent)

        return (mass_kg * u.kg).to(u.M_sun).value


def get_radius(line: str) -> float | None:
    """Get the radius of the body in AU."""

    match = re.search(r"Radius\s*\(\s*(km)\s*\)\s*=\s*([\d\.]+)", line, re.IGNORECASE)

    if match:
        value_km = float(match.group(2))

        return (value_km * u.km).to(u.AU).value


def get_temp(line: str) -> float | None:
    """Get the mean temperature of the body in Kelvin."""

    match = re.search(
        r"(?:Mean Temperature|Mean surface temp \(Ts\)|Atmos\. temp\. \(1 bar\))[^=]*=\s*([\d\.]+)",
        line,
        re.IGNORECASE,
    )

    if match:
        value = float(match.group(1))

        return value


def get_obliquity(line: str) -> float | None:
    """Get the obliquity of the body in degrees."""

    match = re.search(
        r"obliquity[^\d]*(?:\[[^\]]*\])?\s*=\s*([\d\.]+)|\w+,\s*deg\s*=\s*([\d\.]+)",
        line,
        re.IGNORECASE,
    )

    if match:
        value = float(match.group(1))

        return value


def get_rotation(line: str) -> float | None:
    """Get the rotation of the body in rad/s."""

    match = re.search(
        r"rot\.?\s*rat[e]?,?\s*[\(\,]?\s*rad/s\s*[\)]?\s*=\s*(-?[\d\.]+)",
        line,
        re.IGNORECASE,
    )

    if match:
        value = float(match.group(1))

        return value


def get_density(line: str) -> float | None:
    """Get the density of the body in g/cm^3."""

    match = re.search(  # it can also wrongly be '(g cm^-3)'
        r"Density\s*.\s*(g\s*\/?\s*cm\^\-?3)\s*.\s*=\s*([\d\.]+)",
        line,
        re.IGNORECASE,
    )

    if match:
        value = float(match.group(2))

        return value


corrected_ids = {
    "Ceres": 2000001,
}


def get_id_and_name(line: str) -> [str, int]:
    """Get the ID and name of the body"""

    match = re.search(
        r"\s+(\S+)\s+(\d+)(?:\s*/\s*\d+)?$",
        line,
        re.IGNORECASE,
    )

    if match:
        name = match.group(1)
        id = int(match.group(2))

        return id, name

    smallbody_match = re.search(
        r"\s+(\d+)\s+([\w-]+)",
        line,
        re.IGNORECASE,
    )

    if smallbody_match:
        id = int(smallbody_match.group(1))
        name = smallbody_match.group(2)

        # Some IDs need correction, for example Ceres
        if name in corrected_ids:
            id = corrected_ids[name]

        return id, name


def get_data(text: str):
    start_marker = "***"
    end_marker = "\n\n\n"
    start_index = text.find(start_marker)
    end_index = text.find(end_marker)

    properties_section = text[start_index:end_index].strip()

    data = {}

    for i, line in enumerate(properties_section.splitlines()):
        if i == 1:
            id, name = get_id_and_name(line)

            data["id"] = id
            data["name"] = name
            data["color"] = color_map[name]

            continue

        line = line.strip()
        if not line:
            continue

        # If there already is a value for a given property, skip it.
        # The first value *should* have been what we want.

        # Note: small bodies such as Ceres, Eris, etc. do not seem to have these properties listed.
        mass = get_mass(line)
        if mass is not None and "mass" not in data:
            data["mass"] = mass

        radius = get_radius(line)
        if radius is not None and "radius" not in data:
            data["radius"] = radius

        temp = get_temp(line)
        if temp is not None and "temperature" not in data:
            data["temperature"] = temp

        obliquity = get_obliquity(line)
        if obliquity is not None and "obliquity" not in data:
            data["obliquity"] = obliquity

        density = get_density(line)
        if density is not None and "density" not in data:
            data["density"] = density

        rotation = get_rotation(line)
        if rotation is not None and "rotation" not in data:
            data["rotation"] = rotation

    return data


def get_initial_vectors(text: str):
    start = text.find("$$SOE") + len("$$SOE")
    end = text.find("$$EOE")

    if start == -1 or end == -1:
        return None

    data_section = text[start:end].strip()
    lines = data_section.split("\n")

    if lines:
        first_line = lines[0]
        elements = first_line.split(",")
        position = [float(elements[2]), float(elements[3]), float(elements[4])]
        velocity = [float(elements[5]), float(elements[6]), float(elements[7])]

        return {
            "position": {"x": position[0], "y": position[1], "z": position[2]},
            "velocity": {"x": velocity[0], "y": velocity[1], "z": velocity[2]},
        }

    raise ValueError("Could not find initial vectors")


all_bodies_data = []
for body_type, body_list in bodies:
    for body in body_list:
        id = body[0]
        moons = body[1]

        # 2440400.5: 2025-01-01 00:00:00 TDB
        # location="500@10" -> use the Sun as the center
        __body__ = Horizons(id=id, epochs=2440400.5, location="500@10")

        vector_data = get_initial_vectors(__body__.vectors_async().text)
        body_data = get_data(__body__.ephemerides_async().text)

        __data__ = {}

        merged_data = body_data | vector_data

        metadata = {
            "id": merged_data.pop("id", None),
            "name": merged_data.pop("name", None),
            "color": merged_data.pop("color", None)
        }

        __data__["data"] = merged_data
        __data__["metadata"] = metadata

        all_bodies_data.append(__data__)

        print(
            f"<{body_type.capitalize()}>",
            __data__["metadata"]["id"],
            __data__["metadata"]["name"],
        )
        print(f"{json.dumps(__data__["data"], indent=4)}")

        path = os.path.join(os.path.dirname(__file__), "responses")
        if not os.path.exists(path):
            os.mkdir(path)

        with open(
            os.path.join(path, f"{__data__["metadata"]["name"]} – {id}.txt"), "w"
        ) as f:
            ephemerides = __body__.ephemerides_async()
            elements = __body__.elements_async()
            vectors = __body__.vectors_async()

            f.write(f"EPHEMERIDES REQUEST URL: {ephemerides.url}\n\n")
            f.write(ephemerides.text)

            f.write(f"ELEMENTS REQUEST URL: {elements.url}\n\n")
            f.write(elements.text)

            f.write(f"VECTORS REQUEST URL: {vectors.url}\n\n")
            f.write(vectors.text)


with open(os.path.join(os.path.dirname(__file__), "compiled_data.json"), "w") as f:
    f.write(json.dumps(all_bodies_data, indent=4))
