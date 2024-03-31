<script>
    import { Dialog } from 'bits-ui';
    import { fade } from 'svelte/transition';

    export let title = 'Title';
    let open = false;

    function closeDialog() {
        open = false;
    }
</script>

<Dialog.Root bind:open>
    <Dialog.Trigger class="button-small">{title}</Dialog.Trigger>
    <Dialog.Portal>
        <Dialog.Overlay
            transition={fade}
            transitionConfig={{ duration: 150 }}
            class="fixed inset-0 z-50 bg-black/50"
        />
        <Dialog.Content
            class="fixed left-[50%] top-[50%] z-50 grid w-full max-w-[94%]
            translate-x-[-50%] translate-y-[-50%]  rounded-card-lg
            border bg-background p-7 shadow-popover outline-none sm:max-w-lg md:w-full"
        >
            <slot {closeDialog}/>
        </Dialog.Content>
    </Dialog.Portal>
</Dialog.Root>