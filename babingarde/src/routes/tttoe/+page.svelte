<script lang="ts">
    let board: number[]
    let turn: boolean
    let gameover: boolean

    function reset() {
        board = Array(9).fill(0)
        turn = true
        gameover = false
    }

    reset()

    function ndx(x: number, y: number) {
        return x + (3*y)
    }

    function display(x: number) {
        switch (x) {
            case 1:
                return 'X'
            case 2:
                return 'O'
            default:
                return ' '
        }
    }

    function click(x: number, y: number) {
        let p = ndx(x,y)

        if (board[p] != 0 || gameover) 
            return

        board[p] = turn ? 1 : 2
        turn = !turn

        if (win()) {
            gameover = true;
        }
    }

    function win() {
        function scan(f: (x: number,y: number) => number): boolean {
            let cnd = 0;
            for (let y = 0; y < 3; y += 1) {
                cnd = board[f(0,y)]
                if (cnd == 0) 
                    continue
                for (let x = 1; x < 3; x += 1) {
                    if (board[f(x,y)] != cnd)
                        break
                    if (x == 2) {
                        return true
                    }
                }
            }

            return false
        }

        function linear(start: number, [ax,ay]: [number,number]): boolean {
            let cnd = board[start]

            if (cnd == 0) 
                return false

            for (let i = 1; true; i += 1) {
                let p = start + ndx(ax*i,ay*i)
                if (p < 0 ||  p >= board.length)
                    break
                
                if (board[p] != cnd) 
                    return false
            }

            return true
        }

        [0,3,6].forEach((y) => {
            if (linear(y,[1,0]))
                return true
        })
        if (scan((x,y)=>ndx(y,x)))
            return true

        if (linear(0,[1,1]))
            return true
        if (linear(2,[-1,1]))
            return true
    }
</script>

<div class=" max-w-7xl mx-auto">
    <div class="mx-auto w-fit text-center">
        <button on:click={()=>reset()} class="m-1 text-xl ">reset</button>
        <table class="text-center text-zinc-100 text-5xl m-0">
            
            {#each [0,1,2] as y}
                <tr class="m-0 p-0">
                    {#each [0,1,2] as x}
                        <td class="p-1 m-0"><button on:click={()=>click(x,y)} class=" bg-zinc-700 w-24 h-24 m-1 p-0 shadow-lg shadow-zinc-900">{display(board[x + (3*y)])}</button></td>
                    {/each}
                </tr>
            {/each}
        </table>
        {#if gameover}
            <p class="p-1">{!turn? display(1): display(2)} is the winner!</p>
        {/if}
    </div>
</div>