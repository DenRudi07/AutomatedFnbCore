import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

WebUI.callTestCase(findTestCase('CMS/Login'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.delay(3)

WebUI.click(findTestObject('CMS/TeamMemberRole/BtnMenuAccount'))

WebUI.delay(3)

WebUI.click(findTestObject('CMS/TeamMemberRole/MenuTeamMemberRole'))

WebUI.delay(3)

WebUI.click(findTestObject('CMS/TeamMemberRole/BtnAddRole'))

WebUI.delay(3)

WebUI.setText(findTestObject('CMS/TeamMemberRole/LabelRoleName'), 'Automated Role')

WebUI.delay(2)

String Ms = WebUI.verifyTextPresent('Sorry, that name already exists :( , please use another name.', false)

if (Ms.equals('true')) {
    int RN

    RN = ((Math.random() * 500) as int)

    WebUI.setText(findTestObject('CMS/TeamMemberRole/LabelRoleName'), ' ' + RN)
} else {
    WebUI.verifyTextPresent('New team role has been added', true)
}

WebUI.delay(2)

WebUI.check(findTestObject('CMS/TeamMemberRole/Page_member/CheckBoxAll'))

WebUI.click(findTestObject('CMS/TeamMemberRole/BtnSave'))

WebUI.closeBrowser()

