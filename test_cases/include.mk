hello-world:
	$(IN_DEV) $(MLC) \
		--json-file $(TEST_CASES_DIR)/json/hello_world.json \
		--build-dir $(BUILD_DIR)/hello_world \
		--ninja samu \
	&& $(IN_DEV) $(BUILD_DIR)/hello_world/a.out

hello-world2:
	$(IN_DEV) $(MLC) \
		--json-file $(TEST_CASES_DIR)/json/hello_world2.json \
		--build-dir $(BUILD_DIR)/hello_world2 \
		--ninja samu \
		-o hello_world \
	&& $(IN_DEV) $(BUILD_DIR)/hello_world2/hello_world

hello-world-cond:
	$(IN_DEV) $(MLC) \
		--json-file $(TEST_CASES_DIR)/json/hello_world_cond.json \
		--build-dir $(BUILD_DIR)/hello_world_cond \
		--ninja samu \
		-o hello_world \
	&& $(IN_DEV) $(BUILD_DIR)/hello_world_cond/hello_world

.PHONY: hello-world hello-world2 hello-world-cond
