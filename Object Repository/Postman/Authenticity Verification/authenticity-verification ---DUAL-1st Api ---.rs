<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>authenticity-verification ---DUAL-1st Api ---</name>
   <tag></tag>
   <elementGuidId>bddd8305-837b-4054-93e8-efe3e85ce961</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;ucode_data\&quot;: \&quot;Name : PRAJWAL KANOJI\\nMob No. : 7022668166\&quot;,\n\t\&quot;captured_frame_url\&quot;:\&quot;aws_path\&quot;,\n    \&quot;location\&quot;:{\&quot;latitude\&quot;:13.6551139,\&quot;longitude\&quot;:74.6648285},\n    \&quot;captured_frame_path\&quot;:\&quot;Android\\/com.acviss.app\\/7975652354\\/7975652354_17082020181440432.jpg\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>auth-id</name>
      <type>Main</type>
      <value>${Auth Id}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>auth-token</name>
      <type>Main</type>
      <value>${Auth Token}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>device-type</name>
      <type>Main</type>
      <value>ANDROID</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>mobile</name>
      <type>Main</type>
      <value>${mobile_number}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value></value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>linked_user_id</name>
      <type>Main</type>
      <value>1234567</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name></name>
      <type>Main</type>
      <value></value>
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
      <defaultValue>GlobalVariable.Host</defaultValue>
      <description></description>
      <id>2b6a18ea-2bf1-4654-b778-df1161d10851</id>
      <masked>false</masked>
      <name>Host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Auth Id</defaultValue>
      <description></description>
      <id>0908ed57-c89a-49d8-945e-8cb870ff607c</id>
      <masked>false</masked>
      <name>Auth Id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Auth Token</defaultValue>
      <description></description>
      <id>25334e15-64af-499c-b5b8-98242feac37d</id>
      <masked>false</masked>
      <name>Auth Token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.mobile_number</defaultValue>
      <description></description>
      <id>dd56fc94-ce0e-47cf-b090-ab31b1d0468e</id>
      <masked>false</masked>
      <name>mobile_number</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
