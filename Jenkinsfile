pipeline {
    agent any
    options { 
        buildDiscarder logRotator(artifactDaysToKeepStr: '', artifactNumToKeepStr: '', daysToKeepStr: '', numToKeepStr: '5')
        disableConcurrentBuilds()
        parallelsAlwaysFailFast()
    }
    stages {
        stage('Run tests') {
            steps {
                echo """
                Job name: ${JOB_NAME}
                Git branch: ${GIT_BRANCH}
                Git commit: ${GIT_COMMIT}
                """
                script {
                    def params = [
                        [
                            $class: 'StringParameterValue',
                            name: 'dockerimage_compilers',
                            value: "tonlabs/compilers:latest"
                        ],
                        [
                            $class: 'StringParameterValue',
                            name: 'dockerimage_local_node',
                            value: "tonlabs/local-node:latest"
                        ],
                        [
                            $class: 'StringParameterValue',
                            name: 'ton_client_rs_branch',
                            value: "${GIT_BRANCH}"
                        ],
                        [
                            $class: 'StringParameterValue',
                            name: 'ton_client_rs_commit',
                            value: "${GIT_COMMIT}"
                        ],
                        [
                            $class: 'BooleanParameterValue',
                            name: 'RUN_TESTS_ALL',
                            value: false
                        ],
                        [
                            $class: 'BooleanParameterValue',
                            name: 'RUN_TESTS_TON_CLIENT_RS',
                            value: true
                        ],

                    ] 

                    build job: "Integration/integration-tests/master", parameters: params
                }
            }
        }
    }
}
