<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>get-redemption-by-category</name>
   <tag></tag>
   <elementGuidId>bcf81b3b-d981-4148-a60a-7952ef6db816</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;category_id\&quot;:3,\n\t\&quot;redemption_id\&quot;:0\n}&quot;,
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
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>customer-id</name>
      <type>Main</type>
      <value>${customer_id}</value>
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
   <restUrl>${Host}loyalty/get-redemption-by-category/</restUrl>
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
   <variables>
      <defaultValue>GlobalVariable.customer_id</defaultValue>
      <description></description>
      <id>40d9764b-a748-453b-bdaa-986f0463f32f</id>
      <masked>false</masked>
      <name>customer_id</name>
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
