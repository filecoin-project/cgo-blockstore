package main

// unresolved-symbols: https://github.com/golang/go/issues/14985
// Otherwise, rust can't find our exported functions.

/*
#include <stdint.h>
#include "./libcgobs_example.h"
#cgo LDFLAGS: -L. -lcgobs_example  -Wl,-unresolved-symbols=ignore-all
*/
import "C"
import (
	"context"
	"fmt"

	cgobs "github.com/Stebalien/cgo-blockstore"
	"github.com/filecoin-project/lotus/blockstore"
)

func main() {
	bs := blockstore.NewMemory()
	store := cgobs.Register(bs)
	res := C.write_a_block(C.int32_t(store))
	fmt.Println(res)
	keys, err := bs.AllKeysChan(context.Background())
	if err != nil {
		panic(err)
	}
	for c := range keys {
		fmt.Println(c)
	}
}
