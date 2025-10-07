<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update Sport Activity</name>
   <tag></tag>
   <elementGuidId>6bda9890-b302-4a93-857d-b65e2bae9cf0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;sport_category_id\&quot;: 1,\n    \&quot;city_id\&quot;: 3172,\n    \&quot;title\&quot;: \&quot;Sepak bola asik, join yuk\&quot;,\n    \&quot;description\&quot;: \&quot;*MF TANGSEL x MF DEPOK x MF BOGOR*\&quot;,\n    \&quot;slot\&quot;: 100,\n    \&quot;price\&quot;: 70000,\n    \&quot;address\&quot;: \&quot;Lapangan Revo, Jakarta Timur\&quot;,\n    \&quot;activity_date\&quot;: \&quot;2026-08-14\&quot;,\n    \&quot;start_time\&quot;: \&quot;06:00\&quot;,\n    \&quot;end_time\&quot;: \&quot;07:00\&quot;,\n    \&quot;map_url\&quot;:\&quot;https://maps.app.goo.gl/h1AV4bfB2cojJMxK7\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${BEARER_TOKEN}</value>
      <webElementGuid>a9f1ed8b-dbb1-4ac8-a34d-8e33ea912f6a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>b3463226-246e-4b9a-a55e-9a3b46342010</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.2.4</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${BASE_URL}/api/v1/sport-activities/update/1</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.BASE_URL</defaultValue>
      <description></description>
      <id>f3c959fb-6dc5-4e93-a4c4-c28cc2b5f24b</id>
      <masked>false</masked>
      <name>BASE_URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.BEARER_TOKEN</defaultValue>
      <description></description>
      <id>b32350d7-b19d-4d29-9ae6-172ab8705e55</id>
      <masked>false</masked>
      <name>BEARER_TOKEN</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
