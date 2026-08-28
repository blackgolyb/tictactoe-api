TARGET_FILE=tic_tac_toe_api
BUILD_FOLDER=build
ASSETS_FOLDER=assets

ifeq ($(TARGET), )
	TARGET_TYPE_FLAGS=
	TARGET_FOLDER=target
else
	TARGET_TYPE_FLAGS=--target $(TARGET)
	TARGET_FOLDER=target/$(TARGET)
endif

ifeq ($(BUILD_TYPE), )
	BUILD_TYPE_FLAGS=
	TARGET_LOCATION=$(TARGET_FOLDER)/debug
else
	BUILD_TYPE_FLAGS=--$(BUILD_TYPE)
	TARGET_LOCATION=$(TARGET_FOLDER)/$(BUILD_TYPE)
endif

build_target:
	cargo build $(BUILD_TYPE_FLAGS) $(TARGET_TYPE_FLAGS)

.SILENT: pack_build
pack_build:
	mkdir -p $(BUILD_FOLDER) &&\
	cp $(TARGET_LOCATION)/$(TARGET_FILE) $(BUILD_FOLDER) &&\
	cp -r $(ASSETS_FOLDER) $(BUILD_FOLDER)

build: build_target pack_build

.SILENT: run
run: build
	set -o allexport &&\
	source ./env/.env &&\
	set +o allexport &&\
	$(BUILD_FOLDER)/$(TARGET_FILE)

docker_run:
	docker compose -f docker-compose.yaml --profile tailscale up -d

docker_down:
	docker compose -f docker-compose.yaml --profile tailscale down --remove-orphans

docker_local_run:
	docker compose -f docker-compose.yaml -f docker-compose.local.yaml up -d app

docker_local_down:
	docker compose -f docker-compose.yaml -f docker-compose.local.yaml down --remove-orphans

docker_build:
	docker compose -f docker-compose.yaml build

docker_build_x86:
	sh ./scripts/docker.sh x86

docker_build_arm:
	sh ./scripts/docker.sh arm

all: run

.PHONY: all build run pack_build build_target
