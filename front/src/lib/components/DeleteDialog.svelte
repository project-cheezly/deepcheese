<script>
    import { AlertDialog } from 'bits-ui';
    import { fade } from 'svelte/transition';

    let open = false;

    export let id;
</script>

<AlertDialog.Root bind:open>
    <AlertDialog.Trigger class="button-small text-red-600">
        삭제
    </AlertDialog.Trigger>
    <AlertDialog.Portal>
        <AlertDialog.Overlay
            transition={fade}
            transitionConfig={{ duration: 150 }}
            class="fixed inset-0 z-50 bg-black/50"
        />
        <AlertDialog.Content
            class="fixed left-[50%] top-[50%] z-50 grid w-full max-w-[94%]
            translate-x-[-50%] translate-y-[-50%] gap-4 rounded-card-lg
            border bg-background p-7 shadow-popover outline-none sm:max-w-lg md:w-full"
        >
            <h3>정말 삭제하시겠습니까?</h3>
            <p>삭제된 데이터는 복구할 수 없습니다.</p>
            <form method="POST" action="?/delete" class="space-x-4 text-right">
                <input type="hidden" name="id" value="{id}" />
                <button class="button-small" on:click={() => { open=false; }}>취소</button>
                <button class="button-small text-red-600">삭제</button>
            </form>
        </AlertDialog.Content>
    </AlertDialog.Portal>
</AlertDialog.Root>