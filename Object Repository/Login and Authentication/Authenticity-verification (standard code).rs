<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Authenticity-verification (standard code)</name>
   <tag></tag>
   <elementGuidId>339ab93c-e498-4e6f-a613-490fc42e2d47</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;ucode_data\&quot;: \&quot;${ucode}\&quot;,\n\t\&quot;captured_frame_url\&quot;:\&quot;aws_path\&quot;,\n    \&quot;location\&quot;:{\&quot;latitude\&quot;:13.6551139,\&quot;longitude\&quot;:74.6648285},\n    \&quot;captured_frame_path\&quot;:\&quot;Android\\/com.acviss.app\\/7975652354\\/7975652354_17082020181440432.jpg\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>auth-id</name>
      <type>Main</type>
      <value>${authid}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>auth-token</name>
      <type>Main</type>
      <value>${auth_token}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>device-type</name>
      <type>Main</type>
      <value>${devicetype}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>mobile</name>
      <type>Main</type>
      <value>${mobileNum}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${bearertoken}</value>
   </httpHeaderProperties>
   <katalonVersion>8.2.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${Host}authenticity-verification/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.auth_token</defaultValue>
      <description></description>
      <id>0249fac3-4c1b-476f-866c-89c64bdb157d</id>
      <masked>false</masked>
      <name>auth_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.authid</defaultValue>
      <description></description>
      <id>d6b05d9c-4026-4085-a635-4ce8f0f4d6bc</id>
      <masked>false</masked>
      <name>authid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.devicetype</defaultValue>
      <description></description>
      <id>1e408b69-6732-4c9f-9669-72ad1a55ee5d</id>
      <masked>false</masked>
      <name>devicetype</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Host</defaultValue>
      <description></description>
      <id>0e916aa2-37a0-497b-b494-ef5705ae81f3</id>
      <masked>false</masked>
      <name>Host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.mobileNum</defaultValue>
      <description></description>
      <id>658f201a-e7b1-46b1-a9c5-bf4a7a516401</id>
      <masked>false</masked>
      <name>mobileNum</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.bearertoken</defaultValue>
      <description></description>
      <id>b8207e33-7adf-413f-abc7-3a80305d270c</id>
      <masked>false</masked>
      <name>bearertoken</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ucode</defaultValue>
      <description></description>
      <id>24ac031e-89f2-4c8f-8242-097d27f57fa7</id>
      <masked>false</masked>
      <name>ucode</name>
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
