<script>
    import BaseChart from "$lib/components/charts/line/BaseChart.svelte";
    import * as Container from '$lib/components/double-layer-container';
    import { page } from '$app/stores';
    import { onMount } from "svelte";
    import { invalidate } from "$app/navigation";
    import CandleChart from "$lib/components/charts/line/CandleChart.svelte";

    onMount(() => {
        const interval = setInterval(() => {
            invalidate('data:categoryValue');
        }, 1000 * 60);

        return () => {
            clearInterval(interval);
        }
    });

    let categoryValueHistory = [];
    $: {
        let temp = $page.data.categoryValueHistory;
        temp.sort((a, b) => new Date(a.timestamp) - new Date(b.timestamp));

        let test = temp.reduce((acc, cur) => {
            if (!acc.has(cur.category_name)) {
                acc.set(cur.category_name, []);
            } else {
                acc.get(cur.category_name).push(cur);
            }

            return acc;
        }, new Map());

        let categoryProperties = new Map();
        test.forEach((_, k) => {
            categoryProperties.set(k, {
                prevValue: 0,
                inserted: false,
                closeDate: test.get(k)[test.get(k).length - 1].timestamp.toString()
            });
        });

        let prevDate = temp[0].timestamp.toString();
        let test2 = temp.reduce((acc, cur) => {
            let currDate = cur.timestamp.toString();

            if (currDate !== prevDate) {
                categoryProperties.forEach((v, k) => {
                    if (new Date(prevDate).getTime() < new Date(v.closeDate).getTime() && !v.inserted) {
                        let prevAccValue = acc.get(prevDate);
                        acc.set(prevDate, prevAccValue + v.prevValue);
                    }
                    categoryProperties.get(k).inserted = false;
                })

                prevDate = currDate;
            }

            categoryProperties.get(cur.category_name).inserted = true;

            if (!acc.has(currDate)) {
                acc.set(currDate, parseFloat(cur.value));
            } else {
                let prev = acc.get(currDate);
                acc.set(currDate, prev + parseFloat(cur.value));
            }

            categoryProperties.get(cur.category_name).prevValue = parseFloat(cur.value);

            return acc;
        }, new Map());

        test2.forEach((v, k) => {
            categoryValueHistory.push({
                timestamp: new Date(k),
                category_name: "total",
                value: v
            });
        });
    }
</script>

<div class="container">
    <h1 class="main-content-large">분석</h1>
    <div/>
</div>

<Container.Root>
    <Container.Content>
        <div class="main-content col-span-2 md:col-span-1">
            <h2>실시간</h2>
            <CandleChart data={$page.data.realtimeCategoryValueHistory} />
        </div>
        <div class="main-content col-span-2 md:col-span-1">
            <h2>일간</h2>
            <BaseChart data={categoryValueHistory} timeScale="date" />
        </div>
    </Container.Content>
</Container.Root>
