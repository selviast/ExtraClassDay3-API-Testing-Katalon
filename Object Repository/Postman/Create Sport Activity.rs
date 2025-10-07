<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Sport Activity</name>
   <tag></tag>
   <elementGuidId>7a4bcdfa-a5af-4768-b5e3-570941445a3f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;sport_category_id\&quot;: 1234,\n    \&quot;city_id\&quot;: 3172,\n    \&quot;title\&quot;: \&quot;Test \&quot;,\n    \&quot;description\&quot;: \&quot;using invalid credentials\&quot;,\n    \&quot;slot\&quot;: 6,\n    \&quot;price\&quot;: 50000,\n    \&quot;address\&quot;: \&quot;Lapangan Baturan, Surakarta\&quot;,\n    \&quot;activity_date\&quot;: \&quot;2025-09-30\&quot;,\n    \&quot;start_time\&quot;: \&quot;06:00\&quot;,\n    \&quot;end_time\&quot;: \&quot;09:00\&quot;,\n    \&quot;map_url\&quot;:\&quot;https://maps.app.goo.gl/HRyMk2gSAdevnsB76\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${TOKEN}</value>
      <webElementGuid>2bca3eac-6ace-45f1-a6be-640ae45c5045</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>b3422ffb-f6f9-4c4a-996e-28dc88b23315</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>3f73c1da-c1e8-4242-ba6e-cf5ce18122d4</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.2.4</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${baseURL}/api/v1/sport-activities/create</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.baseURL</defaultValue>
      <description></description>
      <id>03ec24d7-833a-4a05-8e0f-87dc31c3fa3b</id>
      <masked>false</masked>
      <name>baseURL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.TOKEN</defaultValue>
      <description></description>
      <id>31c64baf-c63a-4fad-b33d-8807bcb41560</id>
      <masked>false</masked>
      <name>TOKEN</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
