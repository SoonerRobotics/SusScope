<script lang="ts">
    import { onDestroy, onMount } from "svelte";

    import { Feature } from "ol";
    import Map from "ol/Map";
    import View from "ol/View";
    import { Point } from "ol/geom";
    import TileLayer from "ol/layer/Tile";
    import VectorLayer from "ol/layer/Vector";
    import { fromLonLat } from "ol/proj";
    import VectorSource from "ol/source/Vector";
    import Icon from "ol/style/Icon";
    import Style from "ol/style/Style";
    import XYZ from "ol/source/XYZ";

    let {
        data,
        gps_data,
        show_gps,
    }: {
        data: { latitude: number; longitude: number; theta: number }[];
        gps_data: { latitude: number; longitude: number }[];
        show_gps: boolean;
    } = $props();
    let chart: Map;

    onMount(() => {
        const markers = data.map((item) => {
            const f = new Feature({
                geometry: new Point(
                    fromLonLat([item.longitude, item.latitude]),
                ),
            });

            // Set style to a small arrow pointing in the direction of theta
            f.setStyle(
                new Style({
                    image: new Icon({
                        src: "arrow.png",
                        scale: 0.05,
                        rotation: item.theta,
                        crossOrigin: "anonymous",
                    }),
                }),
            );

            return f;
        });

        const gps_markers = gps_data.map((item) => {
            const f = new Feature({
                geometry: new Point(
                    fromLonLat([item.longitude, item.latitude]),
                ),
            });

            // Set style to a small circle
            f.setStyle(
                new Style({
                    image: new Icon({
                        src: "circle.png",
                        scale: 0.005,
                        crossOrigin: "anonymous",
                    }),
                }),
            );

            return f;
        });

        const source = new VectorSource({
            features: markers,
        });

        const gps_source = new VectorSource({
            features: show_gps ? gps_markers : [],
        });

        chart = new Map({
            target: "map",
            layers: [
                new TileLayer({
                    source: new XYZ({
                        url: "https://server.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer/tile/{z}/{y}/{x}",
                        maxZoom: 19,
                    }),
                }),
                new VectorLayer({
                    source: source,
                }),
                new VectorLayer({
                    source: gps_source,
                }),
            ],
            view: new View({
                center: fromLonLat([-97.44199275186871, 35.21014185571627]),
                zoom: 17,
            }),
        });

        // Fit the map to the markers
        const extent = source.getExtent();
        chart.getView().fit(extent, {
            size: chart.getSize(),
            maxZoom: 19,
        });
    });

    onDestroy(() => {
        if (chart) {
            chart.setTarget(undefined);
            chart.dispose();
        }
    });

    $effect(() => {
        // Whenever data changes, update the markers
        const markers = data.map((item) => {
            const f = new Feature({
                geometry: new Point(
                    fromLonLat([item.longitude, item.latitude]),
                ),
            });

            // Set style to a small arrow pointing in the direction of theta
            f.setStyle(
                new Style({
                    image: new Icon({
                        src: "arrow.png",
                        scale: 0.05,
                        rotation: item.theta,
                        crossOrigin: "anonymous",
                    }),
                }),
            );

            return f;
        });

        const gps_markers = gps_data.map((item) => {
            const f = new Feature({
                geometry: new Point(
                    fromLonLat([item.longitude, item.latitude]),
                ),
            });

            // Set style to a small circle
            f.setStyle(
                new Style({
                    image: new Icon({
                        src: "circle.png",
                        scale: 0.005,
                        crossOrigin: "anonymous",
                    }),
                }),
            );

            return f;
        });

        const source = new VectorSource({
            features: markers,
        });

        const gps_source = new VectorSource({
            features: show_gps ? gps_markers : [],
        });

        chart.getLayers().item(1).set("source", source);
        chart.getLayers().item(2).set("source", gps_source);
    });
</script>

<div id="map" class="w-full h-full"></div>
