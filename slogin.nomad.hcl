variable "version" {
  type = string
}

variable "secret_key" {
  type = string
}

variable "postgres_config" {
  type = object({
    username = string,
    password = string,
    db       = string,
  })
}

variable "redis_config" {
  type = object({
    password = string,
    db       = string,
  })
}

job "slogin" {
  datacenters = ["id-dpk"]

  group "slogin" {
    network {
      mode = "bridge"

      port "healthcheck" {}
    }

    service {
      name = "slogin"
      port = "8080"

      connect {
        sidecar_service {
          proxy {
            upstreams {
              destination_name = "redis"
              local_bind_port  = 6379
            }

            upstreams {
              destination_name = "postgres"
              local_bind_port  = 5432
            }
          }
        }

        sidecar_task {
          resources {
            cpu    = 100
            memory = 50
          }
        }
      }

      check {
        name = "Slogin HTTP Check"
        type = "http"
        port = "healthcheck"
        path = "/health/live"

        interval = "30s"
        timeout  = "10s"
        expose   = true
      }
    }

    task "slogin" {
      driver = "docker"

      config {
        image = "ilmannafian/slogin:${var.version}"
      }

      env {
        SECRET_KEY   = var.secret_key
        REDIS_URL    = "redis://:${var.redis_config.password}@${NOMAD_UPSTREAM_ADDR_redis}/${var.redis_config.db}"
        DATABASE_URL = "postgres://${var.postgres_config.username}:${var.postgres_config.password}@${NOMAD_UPSTREAM_ADDR_postgres}/${var.postgres_config.db}"

        HOST = "127.0.0.1"
        PORT = "8080"

        RUST_LOG = "info"
      }

      resources {
        cpu    = 50
        memory = 20
      }
    }
  }
}
