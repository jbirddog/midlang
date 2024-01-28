TESTS := \
	hello_world \
	hello_world2 \
	fabs \
	frexp \
	cmp \
	cond

$(TESTS):
	make TEST_CASE=$@ test-compile test-run && \
	echo ""

integration-tests: $(TESTS)
	@/bin/true

.PHONY: integration-tests $(TESTS)
