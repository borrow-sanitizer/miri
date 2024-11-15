#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct TrackedPointer {
  void *addr;
  uint64_t alloc_id;
  uint64_t tag;
};

extern "C" {

void __bsan_init();

uint64_t __bsan_expose_tag(TrackedPointer ptr);

uint64_t __bsan_retag(TrackedPointer ptr, PlaceKind place_kind);

uint64_t __bsan_retag_fn(TrackedPointer ptr, ProtectorKind protector_kind, PlaceKind place_kind);

uint64_t __bsan_read(TrackedPointer ptr, uint64_t access_size);

uint64_t __bsan_write(TrackedPointer ptr, uint64_t access_size);

uint64_t __bsan_dealloc(TrackedPointer ptr);

TrackedPointer __bsan_alloc(void *addr);

void __bsan_push_stack_frame();

void __bsan_pop_stack_frame();

}  // extern "C"
