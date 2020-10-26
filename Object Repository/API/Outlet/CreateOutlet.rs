<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateOutlet</name>
   <tag></tag>
   <elementGuidId>ab282d65-5863-4b13-b378-715dc12f8e96</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;brandId\&quot;: 1,\n    \&quot;code\&quot;: \&quot;Luwak9991\&quot;,\n    \&quot;name\&quot;: \&quot;Luwak Store\&quot;,\n    \&quot;address\&quot;: \&quot;Gandaria City\&quot;,\n    \&quot;locationId\&quot;: 1,\n    \&quot;longitude\&quot;: 180,\n    \&quot;latitude\&quot;: 80.1,\n    \&quot;email\&quot;: \&quot;outlet@outlet.com\&quot;,\n    \&quot;phoneNumber\&quot;: \&quot;1212\&quot;,\n    \&quot;whatsappNumber\&quot;: \&quot;628324324434\&quot;,\n    \&quot;websiteUrl\&quot;: \&quot;https://example.com\&quot;,\n    \&quot;bookingUrl\&quot;: \&quot;https://chope.com\&quot;,\n    \&quot;orderUrl\&quot;: \&quot;https://esb.com\&quot;,\n    \&quot;openingHours\&quot;: [\n        {\&quot;day\&quot;:\&quot;1\&quot;,\&quot;isAllHour\&quot;: 1, \&quot;isSpecial\&quot;:0, \&quot;start\&quot;:\&quot;08:00\&quot;,\&quot;end\&quot;:\&quot;23:00\&quot;,\&quot;isOpen\&quot;:1},\n        {\&quot;day\&quot;:\&quot;2\&quot;,\&quot;start\&quot;:\&quot;08:00\&quot;,\&quot;end\&quot;:\&quot;23:00\&quot;,\&quot;isOpen\&quot;:1},\n        {\&quot;day\&quot;:\&quot;3\&quot;,\&quot;start\&quot;:\&quot;08:00\&quot;,\&quot;end\&quot;:\&quot;23:00\&quot;,\&quot;isOpen\&quot;:1},\n        {\&quot;day\&quot;:\&quot;4\&quot;,\&quot;start\&quot;:\&quot;08:00\&quot;,\&quot;end\&quot;:\&quot;23:00\&quot;,\&quot;isOpen\&quot;:1},\n        {\&quot;day\&quot;:\&quot;5\&quot;,\&quot;start\&quot;:\&quot;08:00\&quot;,\&quot;end\&quot;:\&quot;23:00\&quot;,\&quot;isOpen\&quot;:1},\n        {\&quot;day\&quot;:\&quot;6\&quot;,\&quot;start\&quot;:\&quot;08:00\&quot;,\&quot;end\&quot;:\&quot;23:00\&quot;,\&quot;isOpen\&quot;:1},\n        {\&quot;day\&quot;:\&quot;7\&quot;,\&quot;start\&quot;:\&quot;08:00\&quot;,\&quot;end\&quot;:\&quot;23:00\&quot;,\&quot;isOpen\&quot;:1}\n    ],\n    \&quot;image\&quot;: [\n        {\n            \&quot;type\&quot;: \&quot;PREVIEW\&quot;,\n            \&quot;url\&quot;: \&quot;files/Outlet/preview-WRADXOMP-1080x606.png\&quot;\n        }\n    ],\n    \&quot;status\&quot;: 1\n    \n}&quot;,
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
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${Token}</value>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.API_URL}/v1/outlet</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Token</defaultValue>
      <description></description>
      <id>ada548c5-4f5d-4396-b884-a295260fe9aa</id>
      <masked>false</masked>
      <name>Token</name>
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
