import {
    areaCoordsStore,
    flightPathResultStore,
    droneStore,
    type FlightPathResult,
} from "$lib/stores/stores";
import { invoke } from "@tauri-apps/api/core";
import { get } from "svelte/store";

export async function generateFlightPath() {
    const area_coordinates = get(areaCoordsStore);
    const drone = get(droneStore);

    if (area_coordinates.length < 3) {
        flightPathResultStore.set(null);
        return;
    }

    if (!drone) {
        console.error("No drone configured");
        return;
    }

    try {
        const flightPathResult = await invoke<FlightPathResult>(
            "generate_flightpath",
            { coords: area_coordinates, drone: drone },
        );
        flightPathResultStore.set(flightPathResult);
    } catch (error) {
        const msg = typeof error === 'string' ? error : JSON.stringify(error);
        flightPathResultStore.set(null);
        // Temporarily show in page title so it's visible
        document.title = "ERROR: " + msg;
        console.error("Flight path error:", msg);
    }
}
