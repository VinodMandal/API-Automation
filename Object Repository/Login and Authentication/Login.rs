<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Login</name>
   <tag></tag>
   <elementGuidId>ac29d72d-229a-4eba-957d-7e991a575cd6</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n   \&quot;medium\&quot;:\&quot;otp\&quot;,\n   \&quot;mobile\&quot;:\&quot;${mobileNum}\&quot;,\n   \&quot;otp\&quot;: \&quot;1111\&quot;,\n    \&quot;country_code\&quot;:\&quot;+91\&quot;,\n    \&quot;otp_token\&quot;: \&quot;${otptoken}\&quot;,\n    \&quot;fcm_token\&quot;:\&quot;d9OOcCoKQn--0MHw-z47ku:APA91bHyj-eUMu965CQv2ZBnfUWkqa9Dq1fPFuYedS-JbYrEQ-VP3jTqzMfIu27t95c_fjJG2eZONsapmavLrC7mVc-BlyXqRQjMkL-BbXMWf1gXJYZrI41tdTd07PYM4neHI8dGFw9U\&quot;\n\n   \n}&quot;,
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
   <katalonVersion>8.2.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${Host}login/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Host</defaultValue>
      <description></description>
      <id>914320c2-e24c-4aa8-a0e5-a95918282682</id>
      <masked>false</masked>
      <name>Host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.authid</defaultValue>
      <description></description>
      <id>97b76b11-25cc-4ad2-b5ee-bad9b25e253b</id>
      <masked>false</masked>
      <name>authid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.auth_token</defaultValue>
      <description></description>
      <id>8da964ba-4902-47f9-86c1-135b1c93f272</id>
      <masked>false</masked>
      <name>auth_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.mobileNum</defaultValue>
      <description></description>
      <id>faecaace-cfd4-49e1-b869-bcbda792875e</id>
      <masked>false</masked>
      <name>mobileNum</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.devicetype</defaultValue>
      <description></description>
      <id>26202475-e630-4f4f-8d18-65957a106e9c</id>
      <masked>false</masked>
      <name>devicetype</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.otptoken</defaultValue>
      <description></description>
      <id>796bc93c-9b1b-45c1-a109-035d66fe5f34</id>
      <masked>false</masked>
      <name>otptoken</name>
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
