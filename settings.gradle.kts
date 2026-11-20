pluginManagement {
    repositories {
        google {
            content {
                includeGroupByRegex("com\\.android.*")
                includeGroupByRegex("com\\.google.*")
                includeGroupByRegex("androidx.*")
            }
        }
        mavenCentral()
        gradlePluginPortal()
    }
}
plugins {
    id("org.gradle.toolchains.foojay-resolver-convention") version "1.0.0"
}
dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        google()
        mavenCentral()
    }
}

rootProject.name = "Linkup"

include(":app")
project(":app").projectDir = file("android/app")

include(":core-ui")
project(":core-ui").projectDir = file("android/core-ui")

include(":core-domain")
project(":core-domain").projectDir = file("android/core-domain")

include(":core-data")
project(":core-data").projectDir = file("android/core-data")

include(":core-network")
project(":core-network").projectDir = file("android/core-network")

include(":feature-chat")
project(":feature-chat").projectDir = file("android/feature-chat")

include(":feature-friends")
project(":feature-friends").projectDir = file("android/feature-friends")

include(":feature-discovery")
project(":feature-discovery").projectDir = file("android/feature-discovery")

include(":feature-settings")
project(":feature-settings").projectDir = file("android/feature-settings")