<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update_Role</name>
   <tag></tag>
   <elementGuidId>f3f41e69-7a5a-40e3-8986-20eb4f48952b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;${nameRole}\&quot;,\n    \&quot;permission\&quot;: [\n        {\n            \&quot;name\&quot;: \&quot;role\&quot;,\n            \&quot;action\&quot;: {\n                \&quot;create\&quot;: ${action},\n                \&quot;read\&quot;: false,\n                \&quot;update\&quot;: false\n            }\n        },\n        {\n            \&quot;name\&quot;: \&quot;brand\&quot;,\n            \&quot;action\&quot;: {\n                \&quot;create\&quot;: ${action},\n                \&quot;read\&quot;: false,\n                \&quot;update\&quot;: false\n            }\n        }\n    ]\n}&quot;,
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
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${GlobalVariable.API_URL}/v1/admin/role/${GlobalVariable.UUID_ROLE}</restUrl>
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
      <description>Token login</description>
      <id>01b6b955-25de-4dea-b8cc-52c45be01913</id>
      <masked>false</masked>
      <name>Token</name>
   </variables>
   <variables>
      <defaultValue>false</defaultValue>
      <description>action role</description>
      <id>680ee09b-7657-467e-9ceb-46a457e5376a</id>
      <masked>false</masked>
      <name>action</name>
   </variables>
   <variables>
      <defaultValue>'Admin Role'</defaultValue>
      <description></description>
      <id>edc78522-a534-48ff-b9bc-e98e219ba06d</id>
      <masked>false</masked>
      <name>nameRole</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
