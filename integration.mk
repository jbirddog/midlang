TESTS := \
	hello_world \
	hello_world2 \
	hello_world_cond \
	math

$(TESTS):
	make TEST_CASE=$@ test-compile test-run && \
	echo ""

integration-tests: $(TESTS)
	@/bin/true

.PHONY: integration-tests $(TESTS)
