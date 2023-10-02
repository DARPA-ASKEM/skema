FROM openjdk:8 as stage0
LABEL snp-multi-stage="intermediate"
LABEL snp-multi-stage-id="259939ec-452b-4e86-94d0-27e52995cb8d"
WORKDIR /skema/text_reading/webapp/app
COPY skema /skema
USER root
RUN ["chmod", "-R", "u=rwX,g=rwX", "/skema/text_reading/webapp/app"]
RUN ["chmod", "u+x,g+x", "/skema/text_reading/webapp/app/bin/webapp"]
RUN ["chmod", "u+x,g+x", "/skema/text_reading/webapp/app/bin/webapp"]

FROM openjdk:8
LABEL MAINTAINER="Keith Alcock <docker@keithalcock.com>"
USER root
RUN id -u nobody 1>/dev/null 2>&1 || (( getent group 0 1>/dev/null 2>&1 || ( type groupadd 1>/dev/null 2>&1 && groupadd -g 0 root || addgroup -g 0 -S root )) && ( type useradd 1>/dev/null 2>&1 && useradd --system --create-home --uid 1001 --gid 0 nobody || adduser -S -u 1001 -G root nobody ))
WORKDIR /skema/text_reading/webapp/app
COPY --from=stage0 --chown=nobody:root /skema/text_reading/webapp/app /skema/text_reading/webapp/app
ENV APP_VERSION="${steps.generate.outputs.tag}"
ENV APPLICATION_SECRET="this-is-not-a-secure-key-please-change-me"
ENV _JAVA_OPTIONS="-Xmx10g -Xms10g -Dfile.encoding=UTF-8"
RUN chmod 775 /skema/text_reading/webapp/app
USER 1001:0
ENTRYPOINT ["/skema/text_reading/webapp/app/bin/webapp"]
CMD []
