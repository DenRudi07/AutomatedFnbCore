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

WebUI.delay(2)

WebUI.click(findTestObject('CMS/TeamMemberRole/BtnMenuAccount'))

WebUI.delay(2)

WebUI.click(findTestObject('CMS/TeamMemberRole/MenuTeamMembers'))

WebUI.click(findTestObject('CMS/TeamMemberRole/MenuTeamMemberRole'))

WebUI.delay(2)

WebUI.click(findTestObject('CMS/TeamMemberRole/BtnEdit'))

WebUI.delay(2)

Checked = WebUI.verifyElementChecked(findTestObject('CMS/TeamMemberRole/CheckBoxAll'), 0)

if (Checked == true) {
    WebUI.uncheck(findTestObject('CMS/TeamMemberRole/Page_member/CheckBoxAll'))

    WebUI.click(findTestObject('CMS/TeamMemberRole/Page_member/CheckBoxAll'))
}

WebUI.click(findTestObject('CMS/TeamMemberRole/BtnSave'))

WebUI.verifyTextPresent('Permission can\'t be empty', true)

WebUI.closeBrowser()

